use std::sync::Mutex;

use anyhow::anyhow;
use chrono::Duration;
use tauri::{AppHandle, Manager};
use timer::{Guard, Timer};

use crate::{
    communication_manager_state::CommunicationManagerState, communication_manager::SerialPortNames, models::packet::Packet, packet_parser_state::{use_packet_parser, PacketParserState}, packet_structure_manager_state::PacketStructureManagerState, state::{communication_manager_state::use_communication_manager, file_handling_state::{use_file_handler, FileHandlingState}}, use_packet_structure_manager
};

pub struct ReceivingState {
    refresh_timer_data: RefreshTimerData,
}

impl ReceivingState {
    pub fn new(app_handle: AppHandle) -> Self {
        let timer = Timer::new();
        let update_task_guard = timer.schedule_repeating(Duration::milliseconds(50), move || {
            match iterate_receiving_loop(
                app_handle.state::<CommunicationManagerState>(),
                app_handle.state::<PacketStructureManagerState>(),
                app_handle.state::<PacketParserState>(),
                app_handle.state::<FileHandlingState>(),
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
/// If new data has been received, it parses it into packets, stores those packets in the data_processor, 
/// and generates new display packets to be sent to the frontend.
/// 
/// # Input
/// The function takes all of the state structs necessary for it to run. 
/// These can be derived from the app handle, and are necessary for storing data between executions of the loop.
/// 
/// # Error
/// If any of the attempted state accesses or processes fail, 
/// the function will return an error in the form of a string.
/// 
/// # Output
/// If the function runs successfully it will return a RefreshAndReadResult struct
/// The struct contains any new ports that have connected to the groundstation, 
/// along with new display packets for graphs, and other displays.
fn iterate_receiving_loop(
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
    ps_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_parser_state: tauri::State<'_, PacketParserState>,
    file_handler_state: tauri::State<'_, FileHandlingState>
) -> Result<RefreshAndReadResult, String> {
    let mut result: RefreshAndReadResult = RefreshAndReadResult {
        new_available_port_names: None,
        parsed_packets: None,
    };
    let mut read_data: Vec<u8> = vec![];

    // ##########################
    // Get Data
    // ##########################
    match use_communication_manager(communication_manager_state, &mut |communication_manager| {
        match communication_manager.get_data(0) {
            Ok(data) => {
                match use_file_handler(&file_handler_state, &mut |file_handler| {
                    match file_handler.write_bytes(data.data_read.clone()) {
                        Err(err) => {
                            return Err(err)
                        },
                        Ok(ok) => Ok(ok),
                    }
                }){
                    Ok(_) => {},
                    Err(err) => return Err(anyhow!(err.to_string())),
                }
            
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

    // ##########################
    // Process data
    // ##########################
    if !read_data.is_empty() {
        match use_packet_parser(packet_parser_state, &mut |packet_parser| {
            use_packet_structure_manager::<(), String>( &ps_manager_state,&mut |ps_manager| {
                use_file_handler(&file_handler_state, &mut |file_handler| {


                    //add data to parser
                    packet_parser.push_data(&read_data, false);
                    //run parser
                    result.parsed_packets = Some(packet_parser.parse_packets(&ps_manager, false));
                    //write to csv
                    for packet in result.parsed_packets.clone().unwrap(){
                        match file_handler.write_packet(packet) {
                            Err(err) => {
                                println!("Somethings wrong with the csv ):");
                                return Err(err)
                            },
                            Ok(_) => {},
                        };
                    }
                    Ok(())

                    
                })
            })
        }) {
            Ok(_) => {}
            Err(message) => return Err(message),
        }
    }

    

    Ok(result)
}