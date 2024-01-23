

use crate::models::packet::{Packet, PacketFieldValue};

#[derive(serde::Serialize, Default, Debug, Clone)]
pub struct DisplayPacket{
    structure_id: usize,
    field_data: Vec<PacketFieldValue>,
}
pub struct DisplayPacketFieldNames{
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
    pub fn add_new_data(&mut self, new_data: &mut Vec<Packet>) -> Vec<DisplayPacket>{
        self.data_list.append(new_data);
        let mut formatted_buffer = vec![];
        for i in 0..new_data.len(){
            let mut curr_display_packet = DisplayPacket::default();

            curr_display_packet.structure_id = new_data[i].structure_id;

            //convert our timestamp into a regular field
            curr_display_packet.field_data.push(PacketFieldValue::SignedLong(new_data[i].timestamp));

            copy_fields(&mut curr_display_packet, &new_data[i]);

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
fn copy_fields(display_packet: &mut DisplayPacket, packet: &Packet){
    for j in 0..packet.field_data.len(){
        display_packet.field_data.push(packet.field_data[j]);
    }
}
#[cfg(test)]
mod tests {
    use crate::models::packet::{Packet, PacketFieldValue};

    ///first test
    #[test]
    fn test_add(){
        let mut test_vector = vec![];
        for i in 0..4{
            let test_packet: Packet = Packet {
                structure_id: 0,
                field_data: vec![PacketFieldValue::UnsignedInteger(i),
                PacketFieldValue::SignedInteger(2),
                PacketFieldValue::UnsignedShort(3),
                PacketFieldValue::SignedShort(4)],
                timestamp: 0,
            };
            test_vector.push(test_packet);
        }
        
        assert!(test_vector[0].structure_id == 0);
    }
}

