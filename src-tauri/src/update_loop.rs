use std::sync::Mutex;

use anyhow::anyhow;
use chrono::Duration;
use tauri::{AppHandle, Manager};
use timer::{Guard, Timer};

use crate::{
    models::packet::Packet,
    packet_parser_state::use_packet_parser, packet_parser_state::PacketParserState,
    packet_structure_manager_state::PacketStructureManagerState, communications::serial_uart::SerialPortNames,
    communication_state::CommunicationManagerState, use_packet_structure_manager, state::communication_state::use_communication_manager,
};

pub struct TimerState {
    refresh_timer_data: RefreshTimerData,
}

impl TimerState {
    pub fn new(app_handle: AppHandle) -> Self {
        let timer = Timer::new();
        let update_task_guard = timer.schedule_repeating(Duration::milliseconds(50), move || {
            match refresh_available_ports_and_read_active_port(
                app_handle.state::<CommunicationManagerState>(),
                app_handle.state::<PacketStructureManagerState>(),
                app_handle.state::<PacketParserState>(),
            ) {
                Ok(result) => {
                    //sends packets to frontend
                    if result.new_available_port_names.is_some() || result.parsed_packets.is_some() {
                        app_handle.emit_all("serial-update", result).unwrap();
                    }
                }
                Err(message) => app_handle.emit_all("error", message).unwrap(),
            };
        });

        return Self {
            refresh_timer_data:RefreshTimerData {
                timer: timer.into(),
                update_task_guard: update_task_guard.into(),
            },
        };
    }

    pub fn destroy(&self) {
        drop(self.refresh_timer_data.timer.lock().unwrap());
        drop(self.refresh_timer_data.update_task_guard.lock().unwrap());
    }
}

struct RefreshTimerData {
    timer: Mutex<Timer>,
    update_task_guard: Mutex<Guard>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefreshAndReadResult {
    pub(crate) new_available_port_names: Option<Vec<SerialPortNames>>,
    pub(crate) parsed_packets: Option<Vec<Packet>>,

}

/// Main body of the Update loop
/// 
/// First makes an attempt to get new updates from the communications manager. 
/// It then checks to see if the communications manager has read any new data. 
/// If new data has been recieved, it parses it into packets, stores those packets in the data_processor, 
/// and generates new display packets to be sent to the frontend.
/// 
/// # Input
/// The function takes all of the state structs neccisary for it to run. 
/// These can be derived from the app handle, and are necissary for storing data between executions of the loop.
/// 
/// # Error
/// If any of the attempted state accesses or processes fail, 
/// the function will return an error in the form of a string.
/// 
/// # Output
/// If the function runs succesfully it will return a RefreshAndReadResult struct
/// The struct contains any new ports that have connected to the groundstation, 
/// along with new display packets for graphs, and other displays.
fn refresh_available_ports_and_read_active_port(
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_parser_state: tauri::State<'_, PacketParserState>,
) -> Result<RefreshAndReadResult, String> {
    let mut result: RefreshAndReadResult = RefreshAndReadResult {
        new_available_port_names: None,
        parsed_packets: None,
    };
    let mut read_data: Vec<u8> = vec![];

    match use_communication_manager(communication_manager_state, &mut |communication_manager| {
        match communication_manager.get_data() {
            Ok(data) => {
                read_data.extend(data.data_read);
                result.new_available_port_names = data.new_ports;
            },
            Err(error) => return Err(anyhow!(error.to_string()))
        }

        Ok(())
    }) {
        Ok(_) => {}
        Err(message) => return Err(message)
    }
    if !read_data.is_empty() {
        match use_packet_parser(packet_parser_state, &mut |packet_parser| {
            use_packet_structure_manager::<(), String>( &packet_structure_manager_state, &mut |packet_structure_manager| {
                packet_parser.push_data(&read_data,false);
                result.parsed_packets = Some(packet_parser.parse_packets(&packet_structure_manager,false));
                Ok(())
            })
        }) {
            Ok(_) => {}
            Err(message) => return Err(message),
        }
    }

    Ok(result)
}