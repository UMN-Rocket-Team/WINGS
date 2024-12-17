use std::sync::Mutex;

use anyhow::{Error, anyhow};
use chrono::Duration;
use tauri::{AppHandle, Manager};
use timer::{Guard, Timer};

use crate::{
    communication_manager::{CommunicationManager, CommunicationManagerState, DeviceName}, models::packet::Packet, state::generic_state::use_struct
};

pub struct MainLoop {
    refresh_timer_data: RefreshTimerData,
}

impl MainLoop {
    pub fn new(app_handle: AppHandle) -> Self {
        let timer = Timer::new();
        let update_task_guard = timer.schedule_repeating(Duration::milliseconds(50), move || {
            match iterate_receiving_loop(
                app_handle.state::<CommunicationManagerState>(),
            ) {
                Ok(result) => {
                    //sends packets to frontend
                    // app_handle.emit_all("serial-update", result).unwrap();
                    if result.new_available_port_names.is_some() || result.parsed_packets.len() != 0{
                        app_handle.emit_all("serial-update", result).unwrap();
                    }
                }
                Err(message) => app_handle.emit_all("error", message.to_string()).unwrap(),
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
    pub(crate) new_available_port_names: Option<Vec<DeviceName>>,
    pub(crate) parsed_packets: Vec<Packet>,
    pub(crate) got_data: bool

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
) -> Result<RefreshAndReadResult, Error> {
    let mut result: RefreshAndReadResult = RefreshAndReadResult {
        new_available_port_names: None,
        parsed_packets: vec![],
        got_data: false,
    };

    // ##########################
    // Get Data
    // ##########################
    match use_struct(&communication_manager_state, &mut |communication_manager: &mut CommunicationManager| {
        result.new_available_port_names = communication_manager.get_all_potential_devices();
        for device in communication_manager.get_devices(){
            match communication_manager.get_data(device,&mut result.parsed_packets) {
                Ok(_) => {},
                Err(err) => {
                    if err.to_string() != "Operation timed out"{
                        eprintln!("coms manager: {}", err);
                    }
                }
            }
        }
        Ok(())
    }) {
        Ok(result) => {match result{
            Ok(_) => {},
            Err(err) => return Err(err),
        }}
        Err(message) => return Err(anyhow!(message))
    }
    Ok(result)
}





#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use crate::{communication_manager, file_handling::log_handlers::FileHandlingState, state::packet_structure_manager_state::default_packet_structure_manager};

    use super::*; // lets the unit tests use everything in this file
    use tauri::Manager;

    #[test]
    fn run_loop_once() {
        let app_handle = tauri::test::mock_builder().setup(|_app| {Ok(())})
            .manage(CommunicationManagerState::default())
            .manage(FileHandlingState::default())
            .build(tauri::generate_context!())
            .expect("failed to build app");
        assert!(iterate_receiving_loop(
            app_handle.state::<CommunicationManagerState>()).is_ok())
    }

    #[test]
    #[ignore]
    //runs the receiving loop with the expectation that one RFD is connected, will print any hardcoded packets(packets in packet_structure_manager_state.rs) that are received.
    fn can_receive_and_parse_data_with_rfds() {
        //init app
        let app_handle = tauri::test::mock_builder().setup(|_app| {Ok(())})
            .manage(CommunicationManagerState::default())
            .manage(FileHandlingState::default())
            .build(tauri::generate_context!())
            .expect("failed to build app");

        let mut new_id= 0;

        //add an rfd device to comms manager
        let _ = use_struct(&app_handle.state::<CommunicationManagerState>(), &mut |communication_manager| {
            new_id = communication_manager.add_serial_device();
        });
        //run the main receiving loop and print if any data is received
        loop{
            let output = iterate_receiving_loop(
                app_handle.state::<CommunicationManagerState>());
            match output{
                Ok(output) => {
                    match output.new_available_port_names {
                        Some(new_ports) => {
                            if new_ports.len() != 0 {
                                println!("{:#?}", new_ports);
                                match use_struct(&app_handle.state::<CommunicationManagerState>(), &mut |communication_manager| {
                                    let _ = communication_manager.init_device(&new_ports[0].name, 57600, new_id);
                                }){
                                    Ok(_) => {},
                                    Err(err) => {println!("{}", err)},
                                }
                            }
                        },
                        None => {},
                    }
                    if output.parsed_packets.len() != 0 {
                        println!("{:#?}", output.parsed_packets);
                    }},
                Err(err) => {println!("{}", err)},
            }
        }
    }

    #[test]
    #[ignore]
    //runs the receiving loop with the expectation that an RFD and teledongle COM14 are connected
    fn can_receive_and_parse_data_with_multiple_rfds() {
        //init app
        let app_handle = tauri::test::mock_builder().setup(|_app| {Ok(())})
            .manage(CommunicationManagerState::default())
            .manage(FileHandlingState::default())
            .build(tauri::generate_context!())
            .expect("failed to build app");

        let mut new_id= 0;
        let mut new_id_2= 0;

        //add an rfd device to comms manager
        let _ = use_struct(&app_handle.state::<CommunicationManagerState>(), &mut |communication_manager| {
            new_id = communication_manager.add_serial_device();
            new_id_2 = communication_manager.add_altus_metrum();
            communication_manager.ps_manager = Arc::new(default_packet_structure_manager());
        });
        //run the main receiving loop and print if any data is received
        loop{
            let output = iterate_receiving_loop(
                app_handle.state::<CommunicationManagerState>());
            match output{
                Ok(output) => {
                    match output.new_available_port_names {
                        Some(new_ports) => {
                            if new_ports.len() != 0 {
                                println!("{:#?}", new_ports);
                                match use_struct(&app_handle.state::<CommunicationManagerState>(), &mut |communication_manager| {
                                    let _ = communication_manager.init_device(&new_ports[0].name, 57600, new_id);
                                    let _ = communication_manager.init_device("COM14", 57600, new_id_2);
                                }){
                                    Ok(_) => {},
                                    Err(err) => {println!("{}", err)},
                                }
                            }
                        },
                        None => {},
                    }
                    if output.parsed_packets.len() != 0 {
                        for packet in output.parsed_packets{
                            println!("{:?} {}", packet.field_data[0], packet.structure_id);//print timestamp and packet structure id
                        }
                    }},
                Err(err) => {println!("{}", err)},
            }
        }
    }
}