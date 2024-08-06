use std::sync::Mutex;

use anyhow::{bail, Error, anyhow};
use chrono::Duration;
use tauri::{AppHandle, Manager};
use timer::{Guard, Timer};

use crate::{
    communication_manager::{CommunicationManager, DeviceName}, models::packet::Packet, packet_parser::PacketParser, state::generic_state::{result_to_string, use_struct, CommunicationManagerState, FileHandlingState, PacketParserState, PacketStructureManagerState}
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
                app_handle.state::<PacketStructureManagerState>(),
                app_handle.state::<PacketParserState>(),
                app_handle.state::<FileHandlingState>(),
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
    ps_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_parser_state: tauri::State<'_, PacketParserState>,
    file_handler_state: tauri::State<'_, FileHandlingState>
) -> Result<RefreshAndReadResult, Error> {
    let mut result: RefreshAndReadResult = RefreshAndReadResult {
        new_available_port_names: None,
        parsed_packets: vec![],
        got_data: false,
    };
    let mut read_data: Vec<u8> = vec![];

    // ##########################
    // Get Data
    // ##########################
    match use_struct(&communication_manager_state, &mut |communication_manager: &mut CommunicationManager| {
        let mut error_counter = 0;
        for device in communication_manager.get_devices(){
            match communication_manager.get_data(device) {
                Ok(mut data) => {
                    if let Some(ports) = result.new_available_port_names.as_mut() {
                        if let Some(new_ports) = data.new_ports.as_mut(){
                            ports.append(new_ports);
                            ports.sort();
                            ports.dedup();
                            println!("append: {:#?}", ports);
                        }
                    } else {
                        result.new_available_port_names = data.new_ports;
                        if result.new_available_port_names.is_some(){
                           println!("replace: {:#?} \n device: {:#?}", result.new_available_port_names, device);
                        }
                    }



                    match use_struct(&file_handler_state, &mut |file_handler| {
                        match file_handler.write_bytes(data.data_read.clone()) {
                            Err(err) => {
                                return Err(err)
                            },
                            Ok(ok) => Ok(ok),
                        }
                    }){
                        Ok(_) => {},
                        Err(err) => {eprintln!("{}", &err); error_counter += 1;},
                    }


                    read_data = data.data_read;//moving data into new array for ownership purposes

                    if !read_data.is_empty() {
                        // println!("{:#?}",read_data);
                        result.got_data = true;
                        match result_to_string(use_struct(&packet_parser_state, &mut |packet_parser: &mut PacketParser| {
                            use_struct( &ps_manager_state,&mut |ps_manager| {
                                use_struct(&file_handler_state, &mut |file_handler| {
                                    //add data to parser
                                    packet_parser.push_data(&read_data, false);
                                    //run parser
                                    result.parsed_packets.extend(packet_parser.parse_packets(&ps_manager, false));
                                    //write to csv
                                    for packet in result.parsed_packets.clone(){
                                        match file_handler.write_packet(packet,ps_manager) {
                                            Err(err) => {
                                                eprintln!("{}", err.context("Somethings wrong with the csv writing"));
                                            },
                                            Ok(_) => {},
                                        }
                                    }
                                })
                            })
                        })) {
                            Ok(_) => {}
                            Err(message) => {bail!(message)},
                        }
                    }


                },
                Err(err) => {
                    if err != "Operation timed out"{
                        eprintln!("coms manager: {}", err);
                    }
                }
            }
        }
        if error_counter != 0 && read_data.is_empty(){
                eprintln!("bailing main loop iteration from previous errors in coms manager");
                bail!("errors in coms manager");
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

    use super::*; // lets the unit tests use everything in this file
    use tauri::Manager;

    #[test]
    fn run_loop_once() {
        let app_handle = tauri::test::mock_builder().setup(|_app| {Ok(())})
            .manage(PacketStructureManagerState::default())
            .manage(CommunicationManagerState::default())
            .manage(PacketParserState::default())
            .manage(FileHandlingState::default())
            .build(tauri::generate_context!())
            .expect("failed to build app");
        assert!(iterate_receiving_loop(
            app_handle.state::<CommunicationManagerState>(),
            app_handle.state::<PacketStructureManagerState>(),
            app_handle.state::<PacketParserState>(),
            app_handle.state::<FileHandlingState>()).is_ok())
    }

    #[test]
    #[ignore]
    //runs the receiving loop with the expectation that one RFD is connected, will print any hardcoded packets(packets in packet_structure_manager_state.rs) that are received.
    fn can_receive_and_parse_data_with_rfds() {
        //init app
        let app_handle = tauri::test::mock_builder().setup(|_app| {Ok(())})
            .manage(PacketStructureManagerState::default())
            .manage(CommunicationManagerState::default())
            .manage(PacketParserState::default())
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
                app_handle.state::<CommunicationManagerState>(),
                app_handle.state::<PacketStructureManagerState>(),
                app_handle.state::<PacketParserState>(),
                app_handle.state::<FileHandlingState>());
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
            .manage(PacketStructureManagerState::default())
            .manage(CommunicationManagerState::default())
            .manage(PacketParserState::default())
            .manage(FileHandlingState::default())
            .build(tauri::generate_context!())
            .expect("failed to build app");

        let mut new_id= 0;
        let mut new_id_2= 0;

        //add an rfd device to comms manager
        let _ = use_struct(&app_handle.state::<CommunicationManagerState>(), &mut |communication_manager| {
            new_id = communication_manager.add_serial_device();
        });
        let _ = use_struct(&app_handle.state::<CommunicationManagerState>(), &mut |communication_manager| {
            new_id_2 = communication_manager.add_altus_metrum();
        });

        //run the main receiving loop and print if any data is received
        loop{
            let output = iterate_receiving_loop(
                app_handle.state::<CommunicationManagerState>(),
                app_handle.state::<PacketStructureManagerState>(),
                app_handle.state::<PacketParserState>(),
                app_handle.state::<FileHandlingState>());
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