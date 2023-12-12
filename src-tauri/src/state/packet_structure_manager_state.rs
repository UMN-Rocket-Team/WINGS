use std::sync::Mutex;

use crate::{
    models::packet_structure::PacketStructure,
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
            id: 0,
            name: String::from("Official Test"),
            fields: vec![],
            delimiters: vec![],
        };
        example_structure.ez_make("ba5eba11 _4 u64 u16 u16 u8 u8 _6 ca11ab1e");
        
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
