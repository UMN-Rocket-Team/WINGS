use std::sync::Mutex;

use crate::{
    models::packet_structure::{PacketStructure, PacketMetadataType},
    state::mutex_utils::use_state_in_mutex,
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
        let mut example_structure = PacketStructure::default();
        example_structure.ez_make("ba5eba11 _4 i64 u16 u16 u8 u8 _4 ca11ab1e");
        example_structure.fields[0].metadata_type = PacketMetadataType::Timestamp;
        example_structure.fields[0].name = "Timestamp".to_owned();
        example_structure.fields[1].name = "rkt_speed".to_owned();
        example_structure.fields[2].name = "rkt_speed_also".to_owned();
        example_structure.fields[3].name = "rkt_budget".to_owned();
        example_structure.fields[4].name = "var8".to_owned();

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
