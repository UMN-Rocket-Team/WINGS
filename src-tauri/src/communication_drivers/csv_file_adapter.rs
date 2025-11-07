// ****
// Written by Rohan R
// Communications device driver for reading csv files
// 
// ****
use std::{fs::{self, File}, io::{stderr, stdout, Read},sync::{Arc,Mutex}};
use anyhow::{bail, Context};
use serde::de::value;
use tauri::{AppHandle, Manager};
use crate::{communication_manager::{CommsIF, DeviceName}, file_handling::config_struct::ConfigStruct, models::packet::{self, Packet, PacketFieldValue}, packet_structure_manager::PacketStructureManager,state::mutex_utils::use_state_in_mutex,};
use super::serial_packet_parser::SerialPacketParser;
use csv::{self, ByteRecord};
const PRINT_PARSING: bool = false;

#[derive(Default)]

pub struct CSVReadDriver {
    file: Option<File>,
    id: usize,
    baud:usize,
    //packet_parser: SerialPacketParser,
    //config: ConfigStruct
    packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
}

impl CommsIF for CSVReadDriver {
    fn new(packet_structure_manager: Arc<Mutex<PacketStructureManager>>) -> Self
    where
        Self: Sized,
    {
        CSVReadDriver {
            file: None,
            baud: 0,
            id: 0,
            packet_structure_manager,
        }
    }
    fn init_device(&mut self,port_name: &str, baud: u32) -> anyhow::Result<()> {
        self.baud = baud as usize;
        match File::open(port_name){
            Ok(new_file) => {
                self.file = Some(new_file); 
                Ok(())
            },
            Err(err) => {
                eprint!("Error: {:?}",err);
                bail!(err)
            },
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
        let packet_id:usize = self.baud;
        let good_file = match &self.file { //make sure the file object is valid
            Some(file) => file,
            None => return Err(anyhow::anyhow!("Invalid File")),
        };
        let mut reader = csv::Reader::from_reader(good_file);
        let mut field_byte_data = ByteRecord::new();
        let headers = reader.headers()?;
        if !reader.read_byte_record(&mut field_byte_data)? {//reads one row of the csv file into the ByteRecord
            return Err(anyhow::anyhow!("Reached End of File"))
        }
        let field_data = match csv::StringRecord::from_byte_record(field_byte_data) { //converts from ByteRecord to string record
            Ok(value) => value,
            Err(_) => return Err(anyhow::anyhow!("Bytefile does not contain valid utf-8")),
        };
        let good_structure = match self.packet_structure_manager.get_packet_structure(packet_id) {
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
        let new_packet = Packet{structure_id:packet_id,field_data:result};
        packet_vector.push(new_packet);
        Ok(())
    }
    
    fn is_init(&self) -> bool {
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
} 

#[cfg(test)]
mod tests {
    use crate::{models::packet_structure::PacketStructure, packet_structure_manager::PacketStructureManager};
    use crate::{
        communication_manager::{CommunicationManager, DeviceName}, models::packet::Packet, state::packet_structure_manager_state::{default_packet_structure_manager}
    };
    use tauri::{AppHandle, Manager};

    use super::*;//lets the unit tests use everything in this file

    // test for basic packet recognition and parsing
    //Succesfully parses a csv file with small positive as long as it is given the right path and packet structure
    #[test]
    fn test_basic_parsing() {
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            size: Some(0),
            byte_defined: true,
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            packet_crc: vec![],
        };
        p_structure.ez_make("u8 u8 u8", &["Height","Speed","Temperature"],false);
        let id = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut csv_read_driver = CSVReadDriver::default();
        let mut result = csv_read_driver.init_device("./test_files/test.csv",id as u32);
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        for packet in packet_vector {
            let field_data = &packet.field_data;
            assert_eq!(field_data[0],PacketFieldValue::Number(1.0));
            assert_eq!(field_data[1],PacketFieldValue::Number(2.0));
            assert_eq!(field_data[2],PacketFieldValue::Number(3.0));
        }
    }
    //test for parsing negative numbers, succeeds as long as the packet structure marks that the data field is for signed values
    #[test]
    fn test_nonpositive_parsing() {
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            size: Some(0),
            byte_defined: true,
            packet_crc: vec![],
        };
        p_structure.ez_make("u8 i8 u8 u8 i8", &["Height","Speed","Temperature","Time","Location"],false);
        let id = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut csv_read_driver = CSVReadDriver::default();
        let mut result = csv_read_driver.init_device("./test_files/test2.csv",id as u32);
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        for packet in packet_vector {
            let field_data = &packet.field_data;
            assert_eq!(field_data[0],PacketFieldValue::Number(9.0));
            assert_eq!(field_data[1],PacketFieldValue::Number(-8.0));
            assert_eq!(field_data[2],PacketFieldValue::Number(47.0));
            assert_eq!(field_data[3],PacketFieldValue::Number(0.0));
            assert_eq!(field_data[4],PacketFieldValue::Number(-25.0));
        }
    }
    // if the structure has n data fields and the csv file has more than n columns the first n data fields are copied into the structure
    #[test]
    fn test_parsing_too_few_columns() {
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            size: Some(0),
            byte_defined: true,
            packet_crc: vec![],
        };
        p_structure.ez_make("u8 i8 u8", &["Height","Speed","Temperature"],false);
        let id = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut csv_read_driver = CSVReadDriver::default();
        let mut result = csv_read_driver.init_device("./test_files/test2.csv",id as u32);
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        for packet in packet_vector {
            let field_data = &packet.field_data;
            assert_eq!(field_data[0],PacketFieldValue::Number(9.0));
            assert_eq!(field_data[1],PacketFieldValue::Number(-8.0));
            assert_eq!(field_data[2],PacketFieldValue::Number(47.0));
        }
    }
    // if the structure has more data fields than the csv file, the parser will throw an error
    #[test]
    fn test_parsing_too_many_columns() {
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            size: Some(0),
            byte_defined: true,
            packet_crc: vec![],
        };
        p_structure.ez_make("u8 u8 u8 u8 u8", &["Height","Speed","Temperature","Time","Location"],false);
        let id = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut csv_read_driver = CSVReadDriver::default();
        let mut result = csv_read_driver.init_device("./test_files/test.csv",id as u32);
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        for packet in packet_vector {
            let field_data = &packet.field_data;
            assert_eq!(field_data[0],PacketFieldValue::Number(1.0));
            assert_eq!(field_data[1],PacketFieldValue::Number(2.0));
            assert_eq!(field_data[2],PacketFieldValue::Number(3.0));
        }
    }
    /*
    #[test]
    fn test_parsing_two_files() {
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
            packet_crc: vec![],
        };
        p_structure.ez_make("u8 u8 u8", &["Height","Speed","Temperature"]);
        let id = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut csv_read_driver = CSVReadDriver::default();
        let mut result = csv_read_driver.init_device("./test_files/test.csv",id as u32,Arc::new(packet_structure_manager));
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        for packet in packet_vector {
            let field_data = &packet.field_data;
            assert_eq!(field_data[0],PacketFieldValue::UnsignedByte(1));
            assert_eq!(field_data[1],PacketFieldValue::UnsignedByte(2));
            assert_eq!(field_data[2],PacketFieldValue::UnsignedByte(3));
        }
        let mut p_structure2 = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
            packet_crc: vec![],
        };
        p_structure2.ez_make("u8 u8 u8", &["Height","Speed","Temperature","Time","Location"]);
        let id2 = packet_structure_manager.register_packet_structure(&mut p_structure2).unwrap();
        result = csv_read_driver.init_device("./test_files/test2.csv", id2 as u32, Arc::new(packet_structure_manager));
        assert!(result.is_ok());
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        let packet2 = &packet_vector[1]; 
        let field_data = &packet2.field_data;
        assert_eq!(field_data[0],PacketFieldValue::UnsignedByte(9));
        assert_eq!(field_data[1],PacketFieldValue::UnsignedByte(8));
        assert_eq!(field_data[2],PacketFieldValue::UnsignedByte(47));
        assert_eq!(field_data[3],PacketFieldValue::UnsignedByte(0));
        assert_eq!(field_data[4],PacketFieldValue::SignedByte(-25));

    }  */
    //parse two rows from the same file
    #[test]
    fn test_parsing_two_rows() {
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            size: Some(0),
            byte_defined: true,
            packet_crc: vec![],
        };
        p_structure.ez_make("i8 i8 i8 i8 i8", &["Height","Speed","Temperature","Time","Location"],false);
        let id = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut csv_read_driver = CSVReadDriver::default();
        let mut result = csv_read_driver.init_device("./test_files/test2.csv",id as u32);
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        let mut packet = &packet_vector[0]; 
        let field_data = &packet.field_data;
        assert_eq!(field_data[0],PacketFieldValue::Number(9.0));
        assert_eq!(field_data[1],PacketFieldValue::Number(8.0));
        assert_eq!(field_data[2],PacketFieldValue::Number(47.0));
        assert_eq!(field_data[3],PacketFieldValue::Number(0.0));
        assert_eq!(field_data[4],PacketFieldValue::Number(-25.0));
        packet = &packet_vector[1];
        assert_eq!(field_data[0],PacketFieldValue::Number(1.0));
        assert_eq!(field_data[1],PacketFieldValue::Number(-2.0));
        assert_eq!(field_data[2],PacketFieldValue::Number(0.0));
        assert_eq!(field_data[3],PacketFieldValue::Number(4.0));
        assert_eq!(field_data[4],PacketFieldValue::Number(0.0));
    }
    //parse big numbers
    #[test]
    fn test_big_num_parsing() {
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            size: Some(0),
            byte_defined: true,
            packet_crc: vec![],
        };
        p_structure.ez_make("u8 u8 u8 u8 u8", &["Height","Speed","Temperature","Time","Location"],false);
        let id = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut csv_read_driver = CSVReadDriver::default();
        let mut result = csv_read_driver.init_device("./test_files/test3.csv",id as u32);
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        for packet in packet_vector {
            let field_data = &packet.field_data;
            assert_eq!(field_data[0],PacketFieldValue::Number(9.0));
            assert_eq!(field_data[1],PacketFieldValue::Number(8.0));
            assert_eq!(field_data[2],PacketFieldValue::Number(47.0));
            assert_eq!(field_data[3],PacketFieldValue::Number(0.0));
            assert_eq!(field_data[4],PacketFieldValue::Number(-25.0));
        }
    }
    //parse decimals
    #[test]
    fn test_decimal_parsing() {
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            size: Some(0),
            byte_defined: true,
            packet_crc: vec![],
        };
        p_structure.ez_make("u8 u8 u8 u8 u8", &["Height","Speed","Temperature","Time","Location"],false);
        let id = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut csv_read_driver = CSVReadDriver::default();
        let mut result = csv_read_driver.init_device("./test_files/test4.csv",id as u32);
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        for packet in packet_vector {
            let field_data = &packet.field_data;
            assert_eq!(field_data[0],PacketFieldValue::Number(9.0));
            assert_eq!(field_data[1],PacketFieldValue::Number(8.0));
            assert_eq!(field_data[2],PacketFieldValue::Number(47.0));
            assert_eq!(field_data[3],PacketFieldValue::Number(0.0));
            assert_eq!(field_data[4],PacketFieldValue::Number(-25.0));
        }
    }
}