#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod packet_parser;
mod serial;

use std::sync::Mutex;

use crate::packet_parser::PacketParser;
use packet_parser::{
    Packet, PacketDelimiter, PacketField, PacketFieldType, PacketMetadataType, PacketStructure,
};
use serial::{RadioTestResult, SerialManager, SerialPortNames};
use tauri::Manager;

#[derive(Default)]
struct SerialManagerState {
    serial_manager: Mutex<SerialManager>,
}

struct PacketParserState {
    packet_parser: Mutex<PacketParser>,
}

impl Default for PacketParserState {
    fn default() -> Self {
        let mut packet_parser = PacketParser::default();

        // Register default test packet structure
        match packet_parser.register_packet_structure(PacketStructure {
            id: 1,
            name: "Test 1".to_string(),
            fields: vec![PacketField {
                index: 0,
                name: String::from("Test Field 1"),
                r#type: PacketFieldType::UnsignedInteger,
                offset_in_packet: 0,
                metadata_type: PacketMetadataType::None,
            }],
            delimiters: vec![PacketDelimiter {
                index: 0,
                name: String::from("Test Delimiter 1"),
                identifier: vec![0xFF, 0xFF, 0xFF, 0xFF],
                offset_in_packet: 4,
            }],
        }) {
            Ok(_) => {}
            Err(_) => {}
        }

        Self {
            packet_parser: Mutex::new(packet_parser),
        }
    }
}

#[derive(serde::Serialize, Debug)]
struct RefreshAndReadResult {
    new_available_port_names: Option<Vec<SerialPortNames>>,
    parsed_packets: Option<Vec<Packet>>,
}

#[tauri::command]
fn refresh_available_ports_and_read_active_port(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    packet_parser_state: tauri::State<'_, PacketParserState>,
) -> Result<RefreshAndReadResult, String> {
    let mut result: RefreshAndReadResult = RefreshAndReadResult {
        new_available_port_names: None,
        parsed_packets: None,
    };
    let mut read_data: Vec<u8> = vec![];

    {
        let usb_manager_result = serial_manager_state.serial_manager.lock();
        let mut usb_manager = match usb_manager_result {
            Ok(usb_manager) => usb_manager,
            Err(error) => return Err(error.to_string()),
        };

        match usb_manager.refresh_available_ports() {
            Ok(new_ports) => {
                if new_ports {
                    result.new_available_port_names = Some(usb_manager.available_port_names.clone())
                }
            }
            Err(error) => return Err(error.description),
        };

        match usb_manager.read_from_active_port(&mut |bytes| read_data.extend(bytes)) {
            Ok(_) => {}
            Err(error) => return Err(error.to_string()),
        }
    }

    if !read_data.is_empty() {
        let packet_parser_result = packet_parser_state.packet_parser.lock();
        let mut packet_parser = match packet_parser_result {
            Ok(packet_parser) => packet_parser,
            Err(error) => return Err(error.to_string()),
        };

        packet_parser.push_data(&read_data);

        result.parsed_packets = Some(packet_parser.parse_packets());
    }

    Ok(result)
}

#[tauri::command]
fn set_active_port(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_usb_manager(serial_manager_state, &mut |usb_manager| {
        usb_manager.set_active_port(port_name)
    })
}

#[tauri::command]
fn set_test_write_port(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_usb_manager(serial_manager_state, &mut |usb_manager| {
        usb_manager.set_test_write_port(port_name)
    })
}

#[tauri::command]
fn set_test_read_port(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_usb_manager(serial_manager_state, &mut |usb_manager| {
        usb_manager.set_test_read_port(port_name)
    })
}

#[tauri::command]
fn test_radios(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
) -> Result<RadioTestResult, String> {
    use_usb_manager(serial_manager_state, &mut |usb_manager| {
        usb_manager.write_test_packet_to_test_port()
    })
}

fn use_usb_manager<ReturnType>(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    callback: &mut dyn FnMut(&mut SerialManager) -> Result<ReturnType, anyhow::Error>,
) -> Result<ReturnType, String> {
    use_state_in_mutex(&serial_manager_state.serial_manager, callback)
}

fn use_packet_parser<ReturnType, ErrorType>(
    packet_parser_state: tauri::State<'_, PacketParserState>,
    callback: &mut dyn FnMut(&mut PacketParser) -> Result<ReturnType, ErrorType>,
) -> Result<ReturnType, String>
where
    ErrorType: std::fmt::Display,
{
    use_state_in_mutex(&packet_parser_state.packet_parser, callback)
}

fn use_state_in_mutex<State, ReturnType, ErrorType>(
    mutex: &Mutex<State>,
    callback: &mut dyn FnMut(&mut State) -> Result<ReturnType, ErrorType>,
) -> Result<ReturnType, String>
where
    ErrorType: std::fmt::Display,
{
    let locked_mutex_result = mutex.lock();

    if locked_mutex_result.is_err() {
        return Err(locked_mutex_result.err().unwrap().to_string());
    }

    let state = &mut *locked_mutex_result.unwrap();

    let result = callback(state);

    match result {
        Ok(return_value) => Ok(return_value),
        Err(error) => Err(error.to_string()),
    }
}

// #[derive(serde::Serialize)]
// enum PacketStructureRegistrationError {
//     MutexError(String),
//     PacketStructureCollisionError(packet_parser::PacketStructureRegistrationError),
// }

// #[tauri::command]
// fn register_packet_structure(packet_parser_state: tauri::State<'_, PacketParserState>, packet_structure: PacketStructure) -> Result<usize, PacketStructureRegistrationError> {
//     let packet_parser_result = packet_parser_state.packet_parser.lock();
//     let mut packet_parser = match packet_parser_result {
//         Ok(packet_parser) => packet_parser,
//         Err(error) => return Err(PacketStructureRegistrationError::MutexError(error.to_string())),
//     };

//     match packet_parser.register_packet_structure(packet_structure) {
//         Ok(structure_id) => Ok(structure_id),
//         Err(error) => Err(PacketStructureRegistrationError::PacketStructureCollisionError(error)),
//     }
// }

fn emit_packet_structure_update_event(
    app_handle: &tauri::AppHandle,
    packet_view_model_indices: Vec<usize>,
    packet_parser: &PacketParser,
) {
    let mut packet_view_models = Vec::with_capacity(packet_view_model_indices.len());
    for packet_view_model_index in packet_view_model_indices {
        packet_view_models.push(&packet_parser.packet_view_models[packet_view_model_index]);
    }

    app_handle
        .emit_all("packet-structures-update", &packet_view_models)
        .unwrap();
}

fn update_packet_structures(
    app_handle: tauri::AppHandle,
    packet_parser_state: tauri::State<'_, PacketParserState>,
    callback: &mut dyn FnMut(&mut PacketParser) -> Result<Vec<usize>, (Vec<usize>, String)>,
) -> Result<(), String> {
    use_packet_parser(packet_parser_state, &mut |packet_parser| {
        let result = callback(packet_parser);
        match result {
            Ok(modified_packet_view_model_indices) => {
                emit_packet_structure_update_event(
                    &app_handle,
                    modified_packet_view_model_indices,
                    packet_parser,
                );
                Ok(())
            }
            Err((modified_packet_view_model_indices, message)) => {
                emit_packet_structure_update_event(
                    &app_handle,
                    modified_packet_view_model_indices,
                    packet_parser,
                );
                Err(message)
            }
        }
    })
}

#[tauri::command]
fn set_field_name(
    app_handle: tauri::AppHandle,
    packet_parser_state: tauri::State<'_, PacketParserState>,
    packet_structure_id: usize,
    field_index: usize,
    name: &str,
) -> Result<(), String> {
    update_packet_structures(app_handle, packet_parser_state, &mut |packet_parser| {
        packet_parser.set_field_name(packet_structure_id, field_index, name)
    })
}

#[tauri::command]
fn set_field_type(
    app_handle: tauri::AppHandle,
    packet_parser_state: tauri::State<'_, PacketParserState>,
    packet_structure_id: usize,
    field_index: usize,
    r#type: PacketFieldType,
) -> Result<(), String> {
    update_packet_structures(app_handle, packet_parser_state, &mut |packet_parser| {
        packet_parser.set_field_type(packet_structure_id, field_index, r#type)
    })
}

#[tauri::command]
fn set_field_metadata_type(
    app_handle: tauri::AppHandle,
    packet_parser_state: tauri::State<'_, PacketParserState>,
    packet_structure_id: usize,
    field_index: usize,
    metadata_type: PacketMetadataType,
) -> Result<(), String> {
    update_packet_structures(app_handle, packet_parser_state, &mut |packet_parser| {
        packet_parser.set_field_metadata_type(packet_structure_id, field_index, metadata_type)
    })
}

#[tauri::command]
fn set_delimiter_name(
    app_handle: tauri::AppHandle,
    packet_parser_state: tauri::State<'_, PacketParserState>,
    packet_structure_id: usize,
    delimiter_index: usize,
    name: &str,
) -> Result<(), String> {
    update_packet_structures(app_handle, packet_parser_state, &mut |packet_parser| {
        packet_parser.set_delimiter_name(packet_structure_id, delimiter_index, name)
    })
}

#[tauri::command]
fn set_delimiter_identifier(
    app_handle: tauri::AppHandle,
    packet_parser_state: tauri::State<'_, PacketParserState>,
    packet_structure_id: usize,
    delimiter_index: usize,
    identifier: &str,
) -> Result<(), String> {
    update_packet_structures(app_handle, packet_parser_state, &mut |packet_parser| {
        packet_parser.set_delimiter_identifier(packet_structure_id, delimiter_index, identifier)
    })
}

#[tauri::command]
fn set_gap_size(
    app_handle: tauri::AppHandle,
    packet_parser_state: tauri::State<'_, PacketParserState>,
    packet_structure_id: usize,
    gap_index: usize,
    size: usize,
) -> Result<(), String> {
    update_packet_structures(app_handle, packet_parser_state, &mut |packet_parser| {
        packet_parser.set_gap_size(packet_structure_id, gap_index, size)
    })
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
            set_gap_size
        ])
        .manage(SerialManagerState::default())
        .manage(PacketParserState::default())
        .setup(move |app| {
            let packet_parser_state = app.state::<PacketParserState>();
            // TODO: is clone necessary here?
            let payload = packet_parser_state
                .packet_parser
                .lock()
                .unwrap()
                .packet_view_models
                .clone();
            let app_handle = app.handle();
            app_handle.clone().once_global("initialized", move |_| {
                app_handle
                    .emit_all("packet-structures-update", &payload)
                    .unwrap();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
