

use crate::{models::{packet::{Packet, PacketFieldValue}, packet_structure::PacketStructure}, packet_structure_manager::PacketStructureManager};

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


    /// Adds give data to the data, list then formats the data and returns it
    /// 
    /// Immediately appends the given packets to the data list, 
    /// Then goes through the packets and converts them into display packets. 
    /// The timestamp from the regular packets gets added onto the display packet as a field, 
    /// timestamp will always be the first field in the display packet.
    /// the display apckets get both returned and added to the "formatted_list" in case they are needed in the future
    /// 
    /// # Input
    /// The new_data vector contains all new packets to be processed. 
    /// The packet_structure_manager is needed to decrypt the packet structures
    /// 
    /// # Output
    /// All newly created DisplayPackets are provided back to the caller in a vector
    pub fn add_new_data(&mut self, new_data: &mut Vec<Packet>, packet_structure_manager: &mut PacketStructureManager) -> Vec<DisplayPacket>{
        self.data_list.append(new_data);
        let mut formatted_buffer = vec![];
        let i = 0;
        while i < new_data.len(){
            let mut curr_display_packet = DisplayPacket::default();

            curr_display_packet.structure_id = new_data[i].structure_id;

            //convert our timestamp into a regular field
            curr_display_packet.field_data.push(PacketFieldValue::SignedLong(new_data[i].timestamp));
            curr_display_packet.field_names.push("Time Recieved".to_owned());

            let packet_structure = &packet_structure_manager.packet_structures[new_data[i].structure_id];
            copy_fields(packet_structure, &mut curr_display_packet, &new_data[i]);

            formatted_buffer.push(curr_display_packet);
        }
        self.formatted_list.append(&mut formatted_buffer);
        return formatted_buffer;
    }
}


/// helper function that copies a packet's fields into the given display_packet
/// 
/// the function iterates through the fields and copies each field and field anme into the display packet
/// 
/// # inputs
/// The packet_structure is neccisary to find the names of the packets fields.
/// The packet is where we get the field data.
/// The display_packet where you want the data to be copied into
fn copy_fields(packet_structure: &PacketStructure, display_packet: &mut DisplayPacket, packet: &Packet){
    let mut j = 0;
    while j < packet_structure.fields.len(){
        display_packet.field_data.push(packet.field_data[j]);
        let field_name = &packet_structure.fields[j].name;
        display_packet.field_names.push(field_name.to_string());
        j += 1;
    }
}



