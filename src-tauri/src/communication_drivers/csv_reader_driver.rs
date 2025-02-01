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
        let packet_id:&Vec<u8> = &vec![0];
        let mut good_file = match self.file { //make sure the file object is valid
            Some(file) => file,
            None => return Err(anyhow::anyhow!("Invalid File")),
        };
        let mut reader = csv::Reader::from_reader(good_file);
        let mut field_byte_data = ByteRecord::new();
        if !reader.read_byte_record(&mut field_byte_data)? {//reads one row of the csv file into the ByteRecord
            return Err(anyhow::anyhow!("Reached End of File"))
        }
        let mut field_data = match csv::StringRecord::from_byte_record(field_byte_data) { //converts from ByteRecord to string record
            Ok(value) => value,
            Err(_) => return Err(anyhow::anyhow!("Bytefile does not contain valid utf-8")),
        };
        let mut good_structure = match self.config.packet_structure_manager.get_packet_structure(packet_id) {
            Ok(structure) => structure, //make sure the packet id returned a real structure
            Err(_) => return Err(anyhow::anyhow!("Invalid Packet")),
        };
        let mut result:Vec<f64> = vec![0.; good_structure.size()]; 
        for delimiter in &good_structure.delimiters {
            let bytes = &delimiter.identifier;
            for i in 0..bytes.len() {
                result[delimiter.offset_in_packet + i] = bytes[i];
                }
            }
        for field in good_structure.fields {
            let given_value = match field_data.get(field.index) {
                Some(value) => value,
                None => return Err(anyhow::anyhow!(format!("Field {} refers to missing index: {}", field.name, field.index)))
            };
            let parsed_value: PacketFieldValue = match field.r#type.make_from_string(given_value){
                Ok(value) => value,
                Err(_)=> return Err(anyhow::anyhow!(format!("Field {} refers to missing index: {}", field.name, field.index))),
            };
            let bytes:Vec<u8> = parsed_value.to_le_bytes();
            for i in 0..field.r#type.size() {
                // guaranteed to not panic due to buffer size calculation previously
                result[field.offset_in_packet + i] = bytes[i];
            }
        }
        let mut new_packet = Packet{structure_id:packet_id,field_data:PacketFieldValue:UnsignedByte(result),field_meta_data:vec![]};
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