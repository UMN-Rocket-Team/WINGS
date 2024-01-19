use anyhow::{Ok,anyhow};

use crate::{models::packet::{Packet, PacketFieldValue}, state::packet_structure_manager_state::{PacketStructureManagerState, use_packet_structure_manager}, packet_structure_manager::PacketStructureManager};

#[derive(Default)]
pub struct DisplayPacket{
    structure_id: usize,
    field_data: Vec<PacketFieldValue>,
    field_names: Vec<String>

}
#[derive(Default)]
pub struct DataProcessor{
    curr_display_packet: DisplayPacket,
}
impl DataProcessor{
    pub fn packet_to_display_packet(&mut self, input: Packet, packet_structure_manager: PacketStructureManager) -> anyhow::Result<()>{
        self.curr_display_packet.field_data = input.field_data;
        self.curr_display_packet.structure_id = input.structure_id;
        let packets_structure = &packet_structure_manager.packet_structures[input.structure_id];
        let mut i = 0;
        while i < packets_structure.fields.len(){
            let field_name = &packets_structure.fields[i].name;
            self.curr_display_packet.field_names.push(field_name.to_string());
            i += 1;
        }
        Ok(())
    }
}