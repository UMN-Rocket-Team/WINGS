use anyhow::{Ok,anyhow};

use crate::{models::packet::{Packet, PacketFieldValue}, packet_structure_manager::PacketStructureManager};

#[derive(Default)]
pub struct DisplayPacket{
    structure_id: usize,
    field_data: Vec<PacketFieldValue>,
    field_names: Vec<String>

}
#[derive(Default)]
pub struct DataProcessor{
    pub data_list: Vec<Packet>,
    pub output_buffer: Vec<DisplayPacket>,
}
impl DataProcessor{
    pub fn add_new_data(&mut self, new_data: &mut Vec<Packet>, packet_structure_manager: PacketStructureManager){
        self.data_list.append(new_data);
        let i = 0;
        while i < new_data.len(){
            _ = self.packet_to_display_packet(&new_data[i], &packet_structure_manager);
        }
    }
    fn packet_to_display_packet(&mut self, input: &Packet, packet_structure_manager: &PacketStructureManager) -> anyhow::Result<()>{
        let mut curr_display_packet = DisplayPacket::default();
        let i = 0;
        while i < input.field_data.len() {
            curr_display_packet.field_data.push(input.field_data[i]);
        }
        curr_display_packet.structure_id = input.structure_id;
        let packets_structure = &packet_structure_manager.packet_structures[input.structure_id];
        let mut i = 0;
        while i < packets_structure.fields.len(){
            let field_name = &packets_structure.fields[i].name;
            curr_display_packet.field_names.push(field_name.to_string());
            i += 1;
        }
        self.output_buffer.push(curr_display_packet);
        return Ok(());
    }
}