// ****
// Written by Rohan R
// Communications device driver for reading csv files
// 
// ****
use std::{fs::File, io::Read};
use anyhow::{bail, Context};
use serde::de::value;
use tauri::{AppHandle, Manager};
use crate::{communication_manager::{CommsIF, DeviceName}, file_handling::config_struct::ConfigStruct, models::packet::{self, Packet, PacketFieldValue}, packet_parser::SerialPacketParser, state::generic_state::{get_clone, ConfigState}};
use csv::{self, ByteRecord};
const PRINT_PARSING: bool = false;

#[derive(Default)]

pub struct CSVReadDriver {
    file: Option<File>,
    id: usize,
    //packet_parser: SerialPacketParser,
    config: ConfigStruct

}

impl CommsIF for CSVReadDriver {
    fn init_device(&mut self,port_name: &str, _baud: u32, app_handle: AppHandle) -> anyhow::Result<()> {
        self.config = get_clone(&app_handle.state::<ConfigState>())?;
        match File::open(port_name){
            Ok(new_file) => {
                self.file = Some(new_file); 
                Ok(())
            },
            Err(err) => bail!(err),
        }
    }
    //Wings should never have anything written into it by wings. The package is also useless since it is not a binary file.
    fn write_port(&mut self, packet: &[u8]) -> anyhow::Result<()> {
        Ok(()) 
    }
    //No raw data to get
    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()>{
        Ok(())
    } 
    //Reads a line of data from a csv file into a data packet of a specificed type
    fn parse_device_data(&mut self, raw_data_vector: &mut Vec<u8>, packet_vector: &mut Vec<Packet>) -> anyhow::Result<()> {
        let packet_id:usize = 0;
        let good_file = match &self.file { //make sure the file object is valid
            Some(file) => file,
            None => return Err(anyhow::anyhow!("Invalid File")),
        };
        let mut reader = csv::Reader::from_reader(good_file);
        let mut field_byte_data = ByteRecord::new();
        if !reader.read_byte_record(&mut field_byte_data)? {//reads one row of the csv file into the ByteRecord
            return Err(anyhow::anyhow!("Reached End of File"))
        }
        let field_data = match csv::StringRecord::from_byte_record(field_byte_data) { //converts from ByteRecord to string record
            Ok(value) => value,
            Err(_) => return Err(anyhow::anyhow!("Bytefile does not contain valid utf-8")),
        };
        let good_structure = match self.config.packet_structure_manager.get_packet_structure(packet_id) {
            Ok(structure) => structure, //make sure the packet id returned a real structure
            Err(_) => return Err(anyhow::anyhow!("Invalid Packet")),
        };
        let mut result:Vec<PacketFieldValue> = vec![]; 
        for field in good_structure.fields.iter() {
            let given_value = match field_data.get(field.index) {
                Some(value) => value,
                None => return Err(anyhow::anyhow!(format!("Field {} refers to missing index: {}", field.name, field.index)))
            };
            let parsed_value: PacketFieldValue = match field.r#type.make_from_string(given_value){
                Ok(value) => value,
                Err(_)=> return Err(anyhow::anyhow!(format!("Field {} refers to missing index: {}", field.name, field.index))),
            };
            result.push(parsed_value);
        }
        let new_packet = Packet{structure_id:packet_id,field_data:result,field_meta_data:vec![]};
        packet_vector.push(new_packet);
        Ok(())
    }
    fn get_new_available_ports(&mut self) -> std::option::Option<Vec<DeviceName>> {
        return None;
    }
    fn is_init(&mut self) -> bool {
        self.file.is_some()
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn get_id(&self) -> usize {
        return self.id;
    }
    fn get_type(&self) -> String {
        return "CSVFile".to_owned();
    }
    fn get_device_packets(&mut self, write_buffer: &mut Vec<Packet>) -> anyhow::Result<()> {
        Ok(())
    }
} 

#[cfg(test)]
mod tests {
    use crate::{models::packet_structure::PacketStructure, packet_structure_manager::PacketStructureManager};
    use crate::{
        communication_manager::{CommunicationManager, DeviceName}, models::packet::Packet, state::generic_state::{use_struct, CommunicationManagerState}
    };
    use tauri::{AppHandle, Manager};

    use super::*;//lets the unit tests use everything in this file

    // test for basic packet recognition and parsing
    #[test]
    fn test_basic_parsing() {
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
        };
        p_structure.ez_make("u8 u8 u8", &["Height","Speed","Temperature"]);
        let id = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut csv_read_driver = CSVReadDriver::default();
        let conf_struct = ConfigStruct{default_baud:0,packet_structure_manager:packet_structure_manager};
        let app_handle = tauri::test::mock_builder().setup(|_app| {Ok(())})
        .manage(conf_struct)
        .build(tauri::generate_context!())
        .expect("failed to build app");
        csv_read_driver.init_device("test.csv",0, app_handle);
    }
}