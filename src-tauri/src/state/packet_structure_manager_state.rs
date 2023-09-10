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
    ///The default configuration for a packetStructureManager(the test packet you see when creating a new flight)
    fn default() -> Self {
        let mut packet_structure_manager = PacketStructureManager::default();
        let default_packet_structures = [PacketStructure {
            id: 0,
            name: String::from("Official Test"),
            fields: vec![
                PacketField {
                    index: 0,
                    name: String::from("var8"),
                    r#type: PacketFieldType::UnsignedByte,
                    offset_in_packet: 12,
                    metadata_type: PacketMetadataType::None,
                },
                PacketField {
                    index: 1,
                    name: String::from("var82"),
                    r#type: PacketFieldType::UnsignedByte,
                    offset_in_packet: 13,
                    metadata_type: PacketMetadataType::None,
                },
                PacketField {
                    index: 2,
                    name: String::from("var16"),
                    r#type: PacketFieldType::UnsignedShort,
                    offset_in_packet: 14,
                    metadata_type: PacketMetadataType::None,
                },
                PacketField {
                    index: 3,
                    name: String::from("var162"),
                    r#type: PacketFieldType::UnsignedShort,
                    offset_in_packet: 16,
                    metadata_type: PacketMetadataType::None,
                },
            ],
            delimiters: vec![
                PacketDelimiter {
                    index: 0,
                    name: String::from("start"),
                    offset_in_packet: 0,
                    identifier: 0xE15AADD0u32.to_le_bytes().to_vec(),
                },
                PacketDelimiter {
                    index: 1,
                    name: String::from("end"),
                    offset_in_packet: 18,
                    identifier: 0xFFFFFFFFu32.to_le_bytes().to_vec(),
                },
            ],
        }];

        for mut default_packet_structure in default_packet_structures {
            match packet_structure_manager.register_packet_structure(&mut default_packet_structure)
            {
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
