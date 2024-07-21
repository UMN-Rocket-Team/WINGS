

use crate::{models::packet::PacketFieldValue, packet_structure_manager::PacketStructureManager};

#[derive(serde::Serialize, Default, Debug, Clone)]
pub struct DisplayPacket{
    structure_id: usize,
    field_data: Vec<PacketFieldValue>,
}
#[derive(serde::Serialize, Default)]
pub struct DisplayPacketFieldNames{
    structure_id: usize,
    field_names: Vec<String>
}
#[derive(Default)]
pub struct DataProcessor{
    pub name_list: Vec<DisplayPacketFieldNames>,
}
impl DataProcessor{
    /// Iterates through the packet_structure_manager and generates a list of names for all the fields
    /// 
    /// Handles the special treatment of timestamps
    pub fn generate_display_field_names(&mut self, packet_structure_manager: &PacketStructureManager) -> Result<&Vec<DisplayPacketFieldNames>,(&Vec<DisplayPacketFieldNames>,String)>{
        self.name_list = vec![];
        for i in 0..packet_structure_manager.packet_structures.len(){
            self.name_list.push(DisplayPacketFieldNames::default());
            self.name_list[i].field_names.push("Time Received".to_owned());
            for j in 0..packet_structure_manager.packet_structures[i].fields.len(){
                self.name_list[i].field_names.push(packet_structure_manager.packet_structures[i].fields[j].name.clone());
            }
        }
        Ok(&self.name_list)
    }
}