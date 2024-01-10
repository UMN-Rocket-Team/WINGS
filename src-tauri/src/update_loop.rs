use std::sync::Mutex;

use anyhow::anyhow;
use chrono::Duration;
use tauri::{AppHandle, Manager};
use timer::{Guard, Timer};

use crate::{
    models::packet::Packet,
    packet_parser_state::use_packet_parser, packet_parser_state::PacketParserState,
    packet_structure_manager_state::PacketStructureManagerState, serial_uart::SerialPortNames,
    communication_state::CommunicationManagerState, use_packet_structure_manager, state::communication_state::use_communication_manager
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
                    if result.new_available_port_names != None || result.parsed_packets != None {
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

/// Refreshes list of ports available
/// reads from active ports and returns parsed data
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
            packet_parser.push_data(&read_data);

            use_packet_structure_manager::<(), &str>(
                &packet_structure_manager_state,
                &mut |packet_structure_manager| {
                    Ok(result.parsed_packets =
                        Some(packet_parser.parse_packets(&packet_structure_manager)))
                },
            )
        }) {
            Ok(_) => {}
            Err(message) => return Err(message),
        }
    }

    Ok(result)
}
