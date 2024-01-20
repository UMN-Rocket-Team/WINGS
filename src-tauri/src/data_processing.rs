

use crate::{models::packet::{Packet, PacketFieldValue}, packet_structure_manager::PacketStructureManager};

#[derive(serde::Serialize, Default, Debug, Clone)]
pub struct DisplayPacket{
    structure_id: usize,
    field_data: Vec<PacketFieldValue>,
    field_names: Vec<String>

}
#[derive(Default)]
pub struct DataProcessor{
    pub data_list: Vec<Packet>,
    pub formatted_list: Vec<DisplayPacket>,
}
impl DataProcessor{
    pub fn add_new_data(&mut self, new_data: &mut Vec<Packet>, packet_structure_manager: &mut PacketStructureManager) -> Vec<DisplayPacket>{
        self.data_list.append(new_data);
        let mut formatted_buffer = vec![];
        let i = 0;
        while i < new_data.len(){
            let mut curr_display_packet = DisplayPacket::default();

            curr_display_packet.structure_id = new_data[i].structure_id;

            //convert timestamp into a regular field
            curr_display_packet.field_data.push(PacketFieldValue::SignedLong(new_data[i].timestamp));
            curr_display_packet.field_names.push("Timestamp".to_owned());

            //find field names and add them to the list along with their data structures
            let packets_structure = &packet_structure_manager.packet_structures[new_data[i].structure_id];
            let mut i = 0;
            while i < packets_structure.fields.len(){
                curr_display_packet.field_data.push(new_data[i].field_data[i]);
                let field_name = &packets_structure.fields[i].name;
                curr_display_packet.field_names.push(field_name.to_string());
                i += 1;
            }
            formatted_buffer.push(curr_display_packet);
        }
        self.formatted_list.append(&mut formatted_buffer);
        return formatted_buffer;
    }
}