// This attribute ensures that on Windows, the console window is hidden in release builds.
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod packet_structure_events;

use std::sync::{Arc, Mutex};

use packet_structure_events::send_initial_packet_structure_update_event;

use wings_packet_parser::prelude::*;

use tauri::{Manager};

use crate::commands::{
    communication_commands::{
        add_aim, add_altus_metrum, add_binary_file, add_csv_file, add_featherweight, add_midwest,
        add_rfd, delete_device, init_device_port,
    },
    file_commands::set_read,
    packet_structure_manager_commands::{
        add_delimiter, add_field, add_gap_after, add_packet_structure, delete_packet_structure,
        delete_packet_structure_component, register_empty_packet_structure,
        set_delimiter_identifier, set_delimiter_name, set_field_metadata_type, set_field_name,
        set_field_type, set_gap_size, set_packet_name,
    }
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
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        // Register all command handlers that can be invoked from the frontend
        .invoke_handler(tauri::generate_handler![
            // Device and communication commands
            delete_device,
            init_device_port,
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
            add_csv_file,
            add_binary_file,
            add_aim,
            add_featherweight,
            add_midwest,
            // File read command
            set_read
        ])
        .setup(move |setup_app| {

            let app_handle = setup_app.handle();

            // Manage shared state objects so they can be accessed in commands and event handlers.
            app_handle.manage(default_packet_structure_manager());
            app_handle.manage(Mutex::new(config));
            app_handle.manage(comms);
            app_handle.manage(data);
            Ok(())
        })
        // .setup(move |setup_app| {
        //     let initialization_handler = move |_e| {
        //         // Send initial packet structure update to the frontend.
        //         send_initial_packet_structure_update_event(app_handle.clone());
        //         // Initialize and start the background refresh timer
        //         // Let the tauri app manage the necessary state so that it can be kept alive for the duration of the
        //         // program and accessed upon termination
        //         if app_handle.try_state::<MainLoop>().is_none() {
        //             app_handle.manage(MainLoop::new(app_handle.clone()));
        //         }
        //     };
        //     setup_app.listen_any("initialized", initialization_handler);

        //     Ok(())
        // })
        // Handle window close events to clean up resources.
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                // Timer internals need to manually dropped, do that here at program termination
                // window.app_handle().state::<MainLoop>().destroy()
            }
        })
        .plugin(tauri_plugin_store::Builder::default().build());

        builder.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
