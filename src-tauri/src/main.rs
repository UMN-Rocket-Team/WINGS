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

use anyhow::anyhow;
use packet_structure_events::{
    send_initial_packet_structure_update_event, update_packet_structures,
};
use packet_structure_manager::SetDelimiterIdentifierError;
use packet_structure_manager_state::{use_packet_structure_manager, PacketStructureManagerState};
use serial_manager_state::{use_serial_manager, SerialManagerState};

use packet::Packet;
use packet_parser_state::{use_packet_parser, PacketParserState};
use packet_structure::{PacketFieldType, PacketMetadataType};

use serial::{RadioTestResult, SerialPortNames};
use tauri::Manager;

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RefreshAndReadResult {
    new_available_port_names: Option<Vec<SerialPortNames>>,
    parsed_packets: Option<Vec<Packet>>,
}

#[tauri::command]
fn refresh_available_ports_and_read_active_port(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_parser_state: tauri::State<'_, PacketParserState>,
) -> Result<RefreshAndReadResult, String> {
    let mut result: RefreshAndReadResult = RefreshAndReadResult {
        new_available_port_names: None,
        parsed_packets: None,
    };
    let mut read_data: Vec<u8> = vec![];

    match use_serial_manager(serial_manager_state, &mut |serial_manager| {
        match serial_manager.refresh_available_ports() {
            Ok(new_ports) => {
                if new_ports {
                    result.new_available_port_names =
                        Some(serial_manager.available_port_names.clone())
                }
            }
            Err(error) => return Err(anyhow!(error.description)),
        };

        match serial_manager.read_from_active_port(&mut |bytes| read_data.extend(bytes)) {
            Ok(_) => Ok(()),
            Err(error) => return Err(anyhow!(error.to_string())),
        }
    }) {
        Ok(_) => {}
        Err(message) => return Err(message),
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
            refresh_available_ports_and_read_active_port,
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
            let app_handle = app.handle();
            app_handle.clone().once_global("initialized", move |_| {
                send_initial_packet_structure_update_event(app_handle);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
