#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod models;
mod mutex_utils;
mod packet_parser;
mod packet_structure_events;
mod packet_structure_manager;
mod packet_view_model;
mod serial;
mod state;
mod update_loop;

use packet_structure_events::send_initial_packet_structure_update_event;
use packet_structure_manager_state::{use_packet_structure_manager, PacketStructureManagerState};
use serial_manager_state::{use_serial_manager, SerialManagerState};

use packet_parser_state::PacketParserState;

use state::{packet_parser_state, packet_structure_manager_state, serial_manager_state};
use tauri::Manager;
use update_loop::TimerState;

use crate::commands::{
    packet_structure_manager_commands::{
        add_delimiter, add_field, add_gap_after, add_packet, delete_packet_structure_component,
        register_empty_packet_structure, set_delimiter_identifier, set_delimiter_name,
        set_field_metadata_type, set_field_name, set_field_type, set_gap_size,
    },
    serial_commands::{set_active_port, set_test_read_port, set_test_write_port, test_radios},
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_active_port,
            set_test_write_port,
            set_test_read_port,
            test_radios,
            set_field_name,
            set_field_type,
            set_field_metadata_type,
            set_delimiter_name,
            set_delimiter_identifier,
            set_gap_size,
            add_field,
            add_delimiter,
            add_gap_after,
            delete_packet_structure_component,
            add_packet,
            register_empty_packet_structure
        ])
        .manage(PacketStructureManagerState::default())
        .manage(SerialManagerState::default())
        .manage(PacketParserState::default())
        .setup(move |app| {
            let app_handle_1 = app.handle();
            let app_handle_2 = app.handle();

            app.once_global("initialized", move |_| {
                send_initial_packet_structure_update_event(app_handle_1);

                // Initialize and start the background refresh timer
                // Let the tauri app manage the necessary state so that it can be kept alive for the duration of the
                // program and accessed upon temination
                app_handle_2.manage(TimerState::new(app_handle_2.clone()));
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
