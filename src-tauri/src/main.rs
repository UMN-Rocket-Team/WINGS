#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod models;
mod packet_generator;
mod packet_structure_events;
mod packet_structure_manager;
mod communication_drivers;
mod state;
mod receiving_loop;
mod sending_loop;
mod communication_manager;
mod data_processing;
mod file_handling;

use std::sync::{Arc, Mutex};

use communication_manager::CommunicationManager;
use data_processing::{DataProcessor};
use file_handling::{config_struct::ConfigStruct, log_handlers::FileHandlingState};
use packet_structure_events::send_initial_packet_structure_update_event;

use packet_structure_manager::PacketStructureManager;
use sending_loop::SendingLoopState;
use state::packet_structure_manager_state::default_packet_structure_manager;
use tauri::Manager;
use receiving_loop::MainLoop;

use crate::commands::{
    packet_structure_manager_commands::{
        add_delimiter, add_field, add_gap_after, add_packet_structure, delete_packet_structure,
        delete_packet_structure_component, register_empty_packet_structure,
        set_delimiter_identifier, set_delimiter_name, set_field_metadata_type, set_field_name,
        set_field_type, set_gap_size, set_packet_name,
    },
    communication_commands::{delete_device, init_device_port,add_altus_metrum,add_rfd, add_file_manager,add_aim, add_featherweight},
    file_commands::set_read,
    sending_commands::{start_sending_loop, stop_sending_loop},
};

/// The main function initializes various states and sets up event handlers and plugins for the Tauri
/// application, running this will start the App.
fn main() {
    //initializing all states
    let config = ConfigStruct::default();
    let ps_manager: Arc<Mutex<PacketStructureManager>> = Arc::new(config.packet_structure_manager.clone().into());
    let data = DataProcessor::default_state(ps_manager.clone());
    let comms = CommunicationManager::default_state(ps_manager.clone());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            delete_device, 
            init_device_port,
            start_sending_loop,
            stop_sending_loop,
            set_field_name,
            set_field_type,
            set_field_metadata_type,
            set_delimiter_name,
            set_delimiter_identifier,
            set_gap_size,
            set_packet_name,
            add_field,
            add_delimiter,
            add_gap_after,
            delete_packet_structure_component,
            add_packet_structure,
            register_empty_packet_structure,
            delete_packet_structure,
            add_altus_metrum,
            add_rfd,
            add_file_manager,
            add_aim,
            add_featherweight,
            set_read
        ])
        .manage(default_packet_structure_manager())
        .manage(Mutex::new(config))
        .manage(comms)
        .manage(data)
        .manage(SendingLoopState::default())
        .manage(FileHandlingState::default())
        .setup(move |app| {
            let app_handle_1 = app.handle();
            let app_handle_2 = app.handle();

            app.listen_global("initialized", move |_| {
                send_initial_packet_structure_update_event(app_handle_1.clone());
                // Initialize and start the background refresh timer
                // Let the tauri app manage the necessary state so that it can be kept alive for the duration of the
                // program and accessed upon termination
                if app_handle_2.try_state::<MainLoop>().is_none() {
                    app_handle_2.manage(MainLoop::new(app_handle_2.clone()));
                }
            });

            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { .. } => {
                // Timer internals need to manually dropped, do that here at program termination
                event.window().app_handle().state::<MainLoop>().destroy()
            }
            _ => {}
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
