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
    pub sending_loop_structure: PacketStructure
}

impl Default for PacketStructureManagerState {
    ///The default configuration for a packetStructureManager(the test packet you see when creating a new flight)
    fn default() -> Self {
        // Used for testing the packet editor.
        let mut example_structure = PacketStructure {
            id: 0, // overwritten by register_packet_structure
            name: String::from("UFC Test Packet"),
            fields: vec![
                PacketField {
                    index: 0,
                    name: String::from("timestamp"),
                    r#type: PacketFieldType::UnsignedLong,
                    offset_in_packet: 8,
                    metadata_type: PacketMetadataType::Timestamp,
                },
                PacketField {
                    index: 1,
                    name: String::from("speed"),
                    r#type: PacketFieldType::UnsignedShort,
                    offset_in_packet: 16,
                    metadata_type: PacketMetadataType::None,
                },
                PacketField {
                    index: 2,
                    name: String::from("speed 2"),
                    r#type: PacketFieldType::UnsignedShort,
                    offset_in_packet: 18,
                    metadata_type: PacketMetadataType::None,
                },
                PacketField {
                    index: 3,
                    name: String::from("budget"),
                    r#type: PacketFieldType::UnsignedByte,
                    offset_in_packet: 20,
                    metadata_type: PacketMetadataType::None,
                },
                PacketField {
                    index: 4,
                    name: String::from("var8"),
                    r#type: PacketFieldType::UnsignedByte,
                    offset_in_packet: 21,
                    metadata_type: PacketMetadataType::None,
                },
                PacketField {
                    index: 5,
                    name: String::from("crc"),
                    r#type: PacketFieldType::UnsignedShort,
                    offset_in_packet: 22,
                    metadata_type: PacketMetadataType::None,
                },
            ],
            delimiters: vec![
                PacketDelimiter {
                    index: 0,
                    name: String::from("start"),
                    offset_in_packet: 0,
                    identifier: 0xBA5EBA11u32.to_le_bytes().to_vec(),
                },
                PacketDelimiter {
                    index: 1,
                    name: String::from("packet type"),
                    offset_in_packet: 4,
                    identifier: 0x0010u16.to_le_bytes().to_vec(),
                },
                PacketDelimiter {
                    index: 2,
                    name: String::from("packet length"),
                    offset_in_packet: 6,
                    identifier: 0x30u16.to_le_bytes().to_vec(),
                },
                PacketDelimiter {
                    index: 3,
                    name: String::from("end"),
                    offset_in_packet: 26,
                    identifier: 0xCA11AB1Eu32.to_le_bytes().to_vec(),
                },
            ],
        };

        // Used by sending loop.
        let mut sending_loop_structure = PacketStructure {
            id: 0, // overwritten by register_packet_structure
            name: String::from("Radio Test Packet"),
            fields: vec![
                PacketField {
                    index: 0,
                    name: String::from("Timestamp"),
                    r#type: PacketFieldType::SignedLong,
                    offset_in_packet: 4,
                    metadata_type: PacketMetadataType::Timestamp,
                },
                PacketField {
                    index: 1,
                    name: String::from("Counter"),
                    r#type: PacketFieldType::UnsignedInteger,
                    offset_in_packet: 12,
                    metadata_type: PacketMetadataType::None
                }
            ],
            delimiters: vec![
                PacketDelimiter {
                    index: 0,
                    name: String::from("start"),
                    offset_in_packet: 0,
                    // 0xdeadbeef!
                    identifier: vec![0xde, 0xad, 0xbe, 0xef]
                },
                PacketDelimiter {
                    index: 1,
                    name: String::from("end"),
                    offset_in_packet: 16,
                    // 0xdeadbeef backwards
                    identifier: vec![0xfe, 0xeb, 0xda, 0xed]
                },
            ]
        };

        let mut packet_structure_manager = PacketStructureManager::default();
        packet_structure_manager.register_packet_structure(&mut example_structure)
            .expect("Failed to register example packet");
        Self {
            packet_structure_manager: Mutex::new(packet_structure_manager),
            sending_loop_structure: example_structure
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
