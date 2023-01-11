#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod serial;
mod packet_parser;

use packet_parser::{Packet, PacketStructure, PacketField, PacketDelimiter, PacketMetadataType, PacketFieldType};
use serial::{SerialManager, SerialPortNames, RadioTestResult};
use crate::packet_parser::PacketParser;

#[derive(Default)]
struct SerialManagerState {
  serial_manager: std::sync::Mutex<SerialManager>,
}

struct PacketParserState {
    packet_parser: std::sync::Mutex<PacketParser>,
}

impl Default for PacketParserState {

    fn default() -> Self {
        let mut packet_parser = PacketParser::default();

        // Register default test packet structure
        match packet_parser.register_packet_structure(PacketStructure {
            id: 0,
            name: "Test 1".to_string(),
            fields: vec![
                PacketField{ name: "Test Field 1".to_string(), r#type: PacketFieldType::UnsignedInteger, offset_in_packet: 0, metadata_type: PacketMetadataType::None }
            ],
            delimiters: vec![
                PacketDelimiter{ name: "Test Delimiter 1".to_string(), identifier: vec![ 0xFF, 0xFF, 0xFF, 0xFF ], offset_in_packet: 4 }
            ],
        }) {
            Ok(_) => {},
            Err(_) => {},
        }

        Self { packet_parser: std::sync::Mutex::new(packet_parser) }
    }

}

#[derive(serde::Serialize)]
#[derive(Debug)]
struct RefreshAndReadResult {
    new_available_port_names: Option<Vec<SerialPortNames>>,
    parsed_packets: Option<Vec<Packet>>,
}

#[tauri::command]
async fn refresh_available_ports_and_read_active_port(serial_manager_state: tauri::State<'_, SerialManagerState>,
                                                      packet_parser_state: tauri::State<'_, PacketParserState>) -> Result<RefreshAndReadResult, String> {
    let mut result: RefreshAndReadResult = RefreshAndReadResult { new_available_port_names: None, parsed_packets: None, };
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
            },
            Err(error) => return Err(error.description),
        };

        match usb_manager.read_from_active_port(&mut |bytes| read_data.extend(bytes)) {
            Ok(_) => {},
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
fn set_active_port(serial_manager_state: tauri::State<'_, SerialManagerState>, port_name: &str) -> Result<(), String> {
    use_usb_manager(serial_manager_state, &mut |usb_manager| usb_manager.set_active_port(port_name))
}

#[tauri::command]
fn set_test_write_port(serial_manager_state: tauri::State<'_, SerialManagerState>, port_name: &str) -> Result<(), String> {
    use_usb_manager(serial_manager_state, &mut |usb_manager| usb_manager.set_test_write_port(port_name))
}

#[tauri::command]
fn set_test_read_port(serial_manager_state: tauri::State<'_, SerialManagerState>, port_name: &str) -> Result<(), String> {
    use_usb_manager(serial_manager_state, &mut |usb_manager| usb_manager.set_test_read_port(port_name))
}

#[tauri::command]
fn test_radios(serial_manager_state: tauri::State<'_, SerialManagerState>) -> Result<RadioTestResult, String> {
    use_usb_manager(serial_manager_state, &mut |usb_manager| usb_manager.write_test_packet_to_test_port())
}

fn use_usb_manager<ReturnType>(serial_manager_state: tauri::State<'_, SerialManagerState>, callback: &mut dyn FnMut(&mut SerialManager) -> Result<ReturnType, anyhow::Error>) -> Result<ReturnType, String> {
    let serial_manager_mutex = serial_manager_state.serial_manager.lock();

    if serial_manager_mutex.is_err() {
        return Err(serial_manager_mutex.err().unwrap().to_string());
    }

    let serial_manager = &mut *serial_manager_mutex.unwrap();

    let result = callback(serial_manager);

    match result {
        Ok(return_value) => Ok(return_value),
        Err(error) => Err(error.to_string()),
    }
}

#[derive(serde::Serialize)]
enum PacketStructureRegistrationError {
    MutexError(String),
    PacketStructureCollisionError(packet_parser::PacketStructureRegistrationError),
}

#[tauri::command]
async fn register_packet_structure(packet_parser_state: tauri::State<'_, PacketParserState>, packet_structure: PacketStructure) -> Result<usize, PacketStructureRegistrationError> {
    let packet_parser_result = packet_parser_state.packet_parser.lock();
    let mut packet_parser = match packet_parser_result {
        Ok(packet_parser) => packet_parser,
        Err(error) => return Err(PacketStructureRegistrationError::MutexError(error.to_string())),
    };
    
    match packet_parser.register_packet_structure(packet_structure) {
        Ok(structure_id) => Ok(structure_id),
        Err(error) => Err(PacketStructureRegistrationError::PacketStructureCollisionError(error)),
    }
}

#[tauri::command]
async fn set_field_name(packet_parser_state: tauri::State<'_, PacketParserState>, packet_structure_id: usize, field_index: usize, name: &str) -> Result<(), String> {
    let packet_parser_result = packet_parser_state.packet_parser.lock();
    let mut packet_parser = match packet_parser_result {
        Ok(packet_parser) => packet_parser,
        Err(error) => return Err(error.to_string()),
    };

    Ok(packet_parser.set_field_name(packet_structure_id, field_index, name))
}

#[tauri::command]
async fn set_field_type(packet_parser_state: tauri::State<'_, PacketParserState>, packet_structure_id: usize, field_index: usize, r#type: PacketFieldType) -> Result<(), String> {
    let packet_parser_result = packet_parser_state.packet_parser.lock();
    let mut packet_parser = match packet_parser_result {
        Ok(packet_parser) => packet_parser,
        Err(error) => return Err(error.to_string()),
    };

    Ok(packet_parser.set_field_type(packet_structure_id, field_index, r#type))
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![refresh_available_ports_and_read_active_port, set_active_port, 
            set_test_write_port, set_test_read_port, register_packet_structure, test_radios, set_field_name, set_field_type])
        .manage(SerialManagerState::default())
        .manage(PacketParserState::default())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
