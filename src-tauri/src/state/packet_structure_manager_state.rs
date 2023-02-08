use std::sync::Mutex;

use crate::{
    models::packet_structure::{
        PacketDelimiter, PacketField, PacketFieldType, PacketMetadataType, PacketStructure,
    },
    mutex_utils::use_state_in_mutex,
    packet_structure_manager::PacketStructureManager,
};

pub struct PacketStructureManagerState {
    pub(crate) packet_structure_manager: Mutex<PacketStructureManager>,
}

impl Default for PacketStructureManagerState {
    fn default() -> Self {
        let mut packet_structure_manager = PacketStructureManager::default();
        let default_packet_structures = [
            PacketStructure {
                id: 0,
                name: String::from("Packet 1"),
                fields: vec![
                    PacketField {
                        index: 0,
                        name: String::from("Field 1"),
                        offset_in_packet: 0,
                        r#type: PacketFieldType::UnsignedInteger,
                        metadata_type: PacketMetadataType::None,
                    },
                    PacketField {
                        index: 1,
                        name: String::from("Field 2"),
                        offset_in_packet: 7,
                        r#type: PacketFieldType::SignedByte,
                        metadata_type: PacketMetadataType::None,
                    },
                ],
                delimiters: vec![PacketDelimiter {
                    index: 0,
                    name: String::from("Delimiter 1"),
                    identifier: vec![0xFF, 0xAB, 0x21],
                    offset_in_packet: 4,
                }],
            },
            PacketStructure {
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
            },
        ];

        for default_packet_structure in default_packet_structures {
            match packet_structure_manager.register_packet_structure(default_packet_structure) {
                Ok(_) => {}
                Err(_) => panic!("Failed to register default packet structures!"),
            }
        }

        Self {
            packet_structure_manager: Mutex::new(packet_structure_manager),
        }
    }
}

pub fn use_packet_structure_manager<ReturnType, ErrorType>(
    packet_structure_manager_state: &PacketStructureManagerState,
    callback: &mut dyn FnMut(&mut PacketStructureManager) -> Result<ReturnType, ErrorType>,
) -> Result<ReturnType, String>
where
    ErrorType: std::fmt::Display,
{
    use_state_in_mutex(
        &packet_structure_manager_state.packet_structure_manager,
        callback,
    )
}
