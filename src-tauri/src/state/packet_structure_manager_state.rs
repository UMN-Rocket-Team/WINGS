use std::sync::Mutex;

use crate::{
    models::packet_structure::PacketStructure,
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
        example_structure.ez_make("205f u16 09 u8 i16 i16 i8 i8 i8 i8 i8 i8 i32 i16 i16 i16 i16 i16 i16");
        example_structure.fields[0].name = "Timestamp".to_owned();
        example_structure.fields[1].name = "state".to_owned();
        example_structure.fields[2].name = "v_batt".to_owned();
        example_structure.fields[3].name = "v_pyro".to_owned();
        example_structure.fields[4].name = "sense_1".to_owned();
        example_structure.fields[5].name = "sense_2".to_owned();
        example_structure.fields[6].name = "sense_3".to_owned();
        example_structure.fields[7].name = "sense_4".to_owned();
        example_structure.fields[8].name = "sense_5".to_owned();
        example_structure.fields[9].name = "sense_6".to_owned();
        example_structure.fields[10].name = "ground_pres".to_owned();
        example_structure.fields[11].name = "ground_accel".to_owned();
        example_structure.fields[12].name = "accel_plus_g".to_owned();
        example_structure.fields[13].name = "accel_minus_g".to_owned();
        example_structure.fields[14].name = "acceleration".to_owned();
        example_structure.fields[15].name = "speed".to_owned();
        example_structure.fields[16].name = "height".to_owned();
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
