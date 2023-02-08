#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod mutex_utils;
mod packet;
mod packet_parser;
mod packet_parser_state;
mod packet_structure;
mod packet_structure_events;
mod packet_structure_manager;
mod packet_structure_manager_state;
mod packet_view_model;
mod serial;
mod serial_manager_state;
mod update_loop;

use packet_structure_events::{
    send_initial_packet_structure_update_event, update_packet_structures,
};
use packet_structure_manager::SetDelimiterIdentifierError;
use packet_structure_manager_state::{use_packet_structure_manager, PacketStructureManagerState};
use serial_manager_state::{use_serial_manager, SerialManagerState};

use packet_parser_state::PacketParserState;
use packet_structure::{PacketFieldType, PacketMetadataType};

use serial::RadioTestResult;
use tauri::Manager;
use update_loop::TimerState;

#[tauri::command]
fn set_active_port(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_serial_manager(serial_manager_state, &mut |usb_manager| {
        usb_manager.set_active_port(port_name)
    })
}

#[tauri::command]
fn set_test_write_port(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_serial_manager(serial_manager_state, &mut |usb_manager| {
        usb_manager.set_test_write_port(port_name)
    })
}

#[tauri::command]
fn set_test_read_port(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_serial_manager(serial_manager_state, &mut |usb_manager| {
        usb_manager.set_test_read_port(port_name)
    })
}

#[tauri::command]
fn test_radios(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
) -> Result<RadioTestResult, String> {
    use_serial_manager(serial_manager_state, &mut |usb_manager| {
        usb_manager.write_test_packet_to_test_port()
    })
}

#[tauri::command]
fn set_field_name(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    field_index: usize,
    name: &str,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.set_field_name(packet_structure_id, field_index, name);
            Ok(vec![packet_structure_id])
        },
    )
}

#[tauri::command]
fn set_field_type(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    field_index: usize,
    r#type: PacketFieldType,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.set_field_type(packet_structure_id, field_index, r#type);
            Ok(vec![packet_structure_id])
        },
    )
}

#[tauri::command]
fn set_field_metadata_type(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    field_index: usize,
    metadata_type: PacketMetadataType,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.set_field_metadata_type(
                packet_structure_id,
                field_index,
                metadata_type,
            );
            Ok(vec![packet_structure_id])
        },
    )
}

#[tauri::command]
fn set_delimiter_name(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    delimiter_index: usize,
    name: &str,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.set_delimiter_name(packet_structure_id, delimiter_index, name);
            Ok(vec![packet_structure_id])
        },
    )
}

#[tauri::command]
fn set_delimiter_identifier(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    delimiter_index: usize,
    identifier: &str,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| match packet_structure_manager.set_delimiter_identifier(
            packet_structure_id,
            delimiter_index,
            identifier,
        ) {
            Ok(_) => Ok(vec![packet_structure_id]),
            Err(SetDelimiterIdentifierError::InvalidHexadecimalString(message)) => {
                Err((vec![packet_structure_id], message))
            }
            Err(SetDelimiterIdentifierError::IdentifierCollision(
                colliding_packet_structure_ids,
            )) => Err((
                colliding_packet_structure_ids,
                String::from("Identifiers must be unique between packet structures!"),
            )),
        },
    )
}

#[tauri::command]
fn set_gap_size(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    gap_index: usize,
    size: usize,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.set_gap_size(packet_structure_id, gap_index, size);
            Ok(vec![packet_structure_id])
        },
    )
}

#[tauri::command]
fn add_field(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.add_field(packet_structure_id);
            Ok(vec![packet_structure_id])
        },
    )
}

#[tauri::command]
fn add_delimiter(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.add_delimiter(packet_structure_id);
            Ok(vec![packet_structure_id])
        },
    )
}

#[tauri::command]
fn add_gap_after(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    is_field: bool,
    component_index: usize,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.add_gap_after(packet_structure_id, is_field, component_index);
            Ok(vec![packet_structure_id])
        },
    )
}

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
            add_gap_after
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
