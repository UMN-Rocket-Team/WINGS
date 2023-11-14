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
        packet_structure_manager.register_packet_structure(&mut example_structure).expect("Failed to register example packet");
        packet_structure_manager.register_packet_structure(&mut sending_loop_structure).expect("Failed to register radio test packet");

        Self {
            packet_structure_manager: Mutex::new(packet_structure_manager),
            sending_loop_structure
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
