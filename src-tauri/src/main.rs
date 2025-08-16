// This attribute ensures that on Windows, the console window is hidden in release builds.
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod communication_drivers;
mod communication_manager;
mod data_processing;
mod file_handling;
mod models;
mod packet_generator;
mod packet_structure_events;
mod packet_structure_manager;
mod receiving_loop;
mod sending_loop;
mod state;

use std::sync::{Arc, Mutex};

use communication_manager::CommunicationManager;
use data_processing::DataProcessor;
use file_handling::{config_struct::ConfigStruct, log_handlers::FileHandlingState};
use packet_structure_events::send_initial_packet_structure_update_event;

use packet_structure_manager::PacketStructureManager;
use receiving_loop::MainLoop;
use sending_loop::SendingLoopState;
use state::packet_structure_manager_state::default_packet_structure_manager;
use tauri::Manager;

use crate::commands::{
    communication_commands::{
        add_aim, add_altus_metrum, add_featherweight, add_file_manager, add_rfd, delete_device,
        init_device_port,
    },
    file_commands::set_read,
    packet_structure_manager_commands::{
        add_delimiter, add_field, add_gap_after, add_packet_structure, delete_packet_structure,
        delete_packet_structure_component, register_empty_packet_structure,
        set_delimiter_identifier, set_delimiter_name, set_field_metadata_type, set_field_name,
        set_field_type, set_gap_size, set_packet_name,
    },
    sending_commands::{start_sending_loop, stop_sending_loop},
};

/// The main function initializes various states and sets up event handlers and plugins for the Tauri
/// application, running this will start the App.
fn main() {
    //initializing all states
    let config = ConfigStruct::default();
    let ps_manager: Arc<Mutex<PacketStructureManager>> =
        Arc::new(config.packet_structure_manager.clone().into());
    let data = DataProcessor::default_state(ps_manager.clone());
    let comms = Mutex::new(CommunicationManager::default_state(ps_manager.clone()));

    // Build the Tauri application.
    tauri::Builder::default()
        // Register all command handlers that can be invoked from the frontend
        .invoke_handler(tauri::generate_handler![
            // Device and communication commands
            delete_device,
            init_device_port,
            start_sending_loop,
            stop_sending_loop,
            // Packet structure commands
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
            // Device-specific commands
            add_altus_metrum,
            add_rfd,
            add_file_manager,
            add_aim,
            add_featherweight,
            // File read command
            set_read
        ])
        // Manage shared state objects so they can be accessed in commands and event handlers.
        .manage(default_packet_structure_manager())
        .manage(Mutex::new(config))
        .manage(comms)
        .manage(data)
        .manage(SendingLoopState::default())
        .manage(FileHandlingState::default())
        // Setup hook runs once when the app starts, used for initialization and event listeners.
        .setup(move |app| {
            let app_handle_1 = app.handle();
            let app_handle_2 = app.handle();

            app.listen_global("initialized", move |_| {
                // Send initial packet structure update to the frontend.
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
        // Handle window close events to clean up resources.
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event.event() {
                // Timer internals need to manually dropped, do that here at program termination
                event.window().app_handle().state::<MainLoop>().destroy()
            }
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
