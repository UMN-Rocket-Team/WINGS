#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod models;
mod packet_parser;
mod packet_generator;
mod packet_structure_events;
mod packet_structure_manager;
mod communications;
mod state;
mod update_loop;
mod sending_loop;
mod communications_manager;
mod data_processing;
mod file_handling;

use commands::sending_commands::{start_sending_loop, stop_sending_loop};
use packet_structure_events::send_initial_packet_structure_update_event;
use packet_structure_manager_state::{use_packet_structure_manager, PacketStructureManagerState};
use communication_state::CommunicationManagerState;
use packet_parser_state::PacketParserState;
use data_processor_state::DataProcessorState;
use sending_loop_state::SendingLoopState;

use state::{communication_state, file_handling_state::FileHandlingState, packet_parser_state, packet_structure_manager_state, sending_loop_state,data_processor_state};
use tauri::Manager;
use update_loop::TimerState;

use crate::commands::{
    packet_structure_manager_commands::{
        add_delimiter, add_field, add_gap_after, add_packet_structure, delete_packet_structure,
        delete_packet_structure_component, register_empty_packet_structure,
        set_delimiter_identifier, set_delimiter_name, set_field_metadata_type, set_field_name,
        set_field_type, set_gap_size, set_packet_name,
    },
    communication_commands::{set_active_port, set_test_port}
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_active_port,
            set_test_port,
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
            delete_packet_structure
        ])
        .manage(PacketStructureManagerState::default())
        .manage(CommunicationManagerState::default())
        .manage(PacketParserState::default())
        .manage(SendingLoopState::default())
        .manage(DataProcessorState::default())
        .manage(FileHandlingState::default())
        .setup(move |app| {
            let app_handle_1 = app.handle();
            let app_handle_2 = app.handle();

            app.listen_global("initialized", move |_| {
                send_initial_packet_structure_update_event(app_handle_1.clone());

                // Initialize and start the background refresh timer
                // Let the tauri app manage the necessary state so that it can be kept alive for the duration of the
                // program and accessed upon temination
                if app_handle_2.try_state::<TimerState>().is_none() {
                    app_handle_2.manage(TimerState::new(app_handle_2.clone()));
                }
            });

            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { .. } => {
                // Timer internals need to manually dropped, do that here at program termination
                event.window().app_handle().state::<TimerState>().destroy()
            }
            _ => {}
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
