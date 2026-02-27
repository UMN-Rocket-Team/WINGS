// ****
// Written by Rohan R., Joe A.
// Communications device driver for reading csv files
//
// ****
use crate::{
    communication_manager::CommsIF,
    models::packet::{Packet, PacketFieldValue},
    models::packet_structure::{PacketField, PacketFieldType, PacketStructure},
    packet_structure_manager::{Error as PacketStructureManagerError, PacketStructureManager},
};
use anyhow::bail;
use csv::{self, ByteRecord, StringRecord};
use std::{
    fs::File,
    path::Path,
    sync::{Arc, Mutex},
};

#[derive(Default)]

pub struct CSVReadDriver {
    file: Option<csv::Reader<File>>,
    id: usize,
    packet_structure_id: Option<usize>,
    packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
}

impl CSVReadDriver {
    fn register_packet_structure_from_header(
        &mut self,
        packet_name: &str,
        header: &StringRecord,
    ) -> anyhow::Result<usize> {
        if packet_name.trim().is_empty() {
            return Err(anyhow::anyhow!(
                "CSV file stem (packet name) cannot be empty"
            ));
        }

        if header.is_empty() {
            return Err(anyhow::anyhow!(
                "CSV header must contain at least one field column"
            ));
        }

        let mut packet_structure = PacketStructure::make_default(packet_name.trim().to_owned());
        packet_structure.fields = header
            .iter()
            .enumerate()
            .map(|(index, name)| PacketField {
                index,
                name: name.trim().to_owned(),
                r#type: PacketFieldType::Double,
                offset_in_packet: index * 8,
            })
            .collect();

        let mut manager = self.packet_structure_manager.lock().unwrap();
        let packet_id = match manager.register_packet_structure(&mut packet_structure) {
            Ok(new_id) => new_id,
            Err(PacketStructureManagerError::NameAlreadyRegistered(existing_id)) => {
                let existing_structure = match manager.get_packet_structure_mut(existing_id) {
                    Ok(existing_structure) => existing_structure,
                    Err(err) => return Err(anyhow::anyhow!(err.to_string())),
                };
                existing_structure.byte_defined = false;
                existing_structure.fields = packet_structure.fields;
                existing_structure.delimiters = vec![];
                existing_structure.packet_crc = vec![];
                existing_structure.size = None;
                existing_id
            }
            Err(other_error) => return Err(anyhow::anyhow!(other_error.to_string())),
        };

        Ok(packet_id)
    }
}

impl CommsIF for CSVReadDriver {
    fn new(packet_structure_manager: Arc<Mutex<PacketStructureManager>>) -> Self
    where
        Self: Sized,
    {
        CSVReadDriver {
            file: None,
            packet_structure_id: None,
            id: 0,
            packet_structure_manager,
        }
    }
    fn init_device(&mut self, port_name: &str, _baud: u32) -> anyhow::Result<()> {
        match File::open(port_name) {
            Ok(new_file) => {
                let mut reader = csv::Reader::from_reader(new_file);
                let packet_name = Path::new(port_name)
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .ok_or(anyhow::anyhow!(
                        "Unable to determine packet name from CSV file path"
                    ))?;

                let header = reader
                    .headers()
                    .map_err(|err| anyhow::anyhow!("Failed to read CSV header: {err}"))?
                    .clone();

                let packet_id = self.register_packet_structure_from_header(packet_name, &header)?;

                self.packet_structure_id = Some(packet_id);
                self.file = Some(reader);
                Ok(())
            }
            Err(err) => {
                eprint!("Error: {:?}", err);
                bail!(err)
            }
        }
    }
    //Wings should never have anything written into it by wings. The package is also useless since it is not a binary file.
    fn write_port(&mut self, _: &[u8]) -> anyhow::Result<()> {
        Ok(())
    }
    //No raw data to get
    fn get_device_raw_data(&mut self, _: &mut Vec<u8>) -> anyhow::Result<()> {
        Ok(())
    }
    //Reads a line of data from a csv file into a data packet of a specified type
    fn parse_device_data(
        &mut self,
        _: &mut Vec<u8>,
        packet_vector: &mut Vec<Packet>,
    ) -> anyhow::Result<()> {
        let packet_id = match self.packet_structure_id {
            Some(id) => id,
            None => {
                return Err(anyhow::anyhow!(
                    "CSV device has no registered packet structure"
                ))
            }
        };
        // Borrow the existing reader
        let reader = match &mut self.file {
            Some(r) => r,
            None => return Err(anyhow::anyhow!("Invalid Reader/File")),
        };

        let mut field_byte_data = ByteRecord::new();
        if !reader.read_byte_record(&mut field_byte_data)? {
            self.file = None;
            return Ok(());
        }
        let field_data = match csv::StringRecord::from_byte_record(field_byte_data) {
            //converts from ByteRecord to string record
            Ok(value) => value,
            Err(_) => return Err(anyhow::anyhow!("CSV record does not contain valid utf-8")),
        };
        let mut manager = self.packet_structure_manager.lock().unwrap();
        let good_structure = match manager.get_packet_structure_mut(packet_id) {
            Ok(structure) => structure, //make sure the packet id returned a real structure
            Err(_) => return Err(anyhow::anyhow!("Invalid Packet")),
        };
        let mut result: Vec<PacketFieldValue> = vec![];
        for field in good_structure.fields.iter() {
            let csv_column_index = field.index;
            let given_value = match field_data.get(csv_column_index) {
                Some(value) => value,
                None => {
                    return Err(anyhow::anyhow!(format!(
                        "Field {} refers to missing index: {}",
                        field.name, csv_column_index
                    )))
                }
            };
            let parsed_value: PacketFieldValue =
                match field.r#type.make_from_string(given_value.trim()) {
                    Ok(value) => value,
                    Err(e) => {
                        return Err(anyhow::anyhow!(format!(
                            "Failed to parse value {:?} for field '{}' (index {}): {}",
                            given_value,
                            field.name,
                            field.index,
                            e
                        )))
                    }
                };
            result.push(parsed_value);
        }
        let new_packet = Packet {
            structure_id: packet_id,
            field_data: result,
        };
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
    use crate::packet_structure_manager::PacketStructureManager;

    use super::*; //lets the unit tests use everything in this file

    #[test]
    fn test_registers_packet_name_from_file_stem_and_header_fields() {
        let packet_structure_manager = PacketStructureManager::default();
        let manager_arc = Arc::new(Mutex::new(packet_structure_manager));
        let mut csv_read_driver = CSVReadDriver::new(manager_arc.clone());

        let result = csv_read_driver.init_device("src/test_files/test.csv", 0);
        assert!(result.is_ok());

        let packet_structure_id = csv_read_driver
            .packet_structure_id
            .expect("packet structure id should be set after init");

        let mut manager = manager_arc.lock().unwrap();
        let structure = manager
            .get_packet_structure_mut(packet_structure_id)
            .expect("packet structure should exist");

        assert_eq!(structure.name, "test");
        assert_eq!(structure.fields.len(), 3);
        assert_eq!(structure.fields[0].name, "Category1");
        assert_eq!(structure.fields[1].name, "Category2");
        assert_eq!(structure.fields[2].name, "Category3");
    }

    // test for basic packet recognition and parsing
    //Succesfully parses a csv file with small positive as long as it is given the right path and packet structure
    #[test]
    fn test_basic_parsing() {
        let packet_structure_manager = PacketStructureManager::default();
        let manager_arc = Arc::new(Mutex::new(packet_structure_manager));
        let mut csv_read_driver = CSVReadDriver::new(manager_arc);
        let mut result = csv_read_driver.init_device("src/test_files/test.csv", 0);
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        for packet in packet_vector {
            let field_data = &packet.field_data;
            println!("{:?}", field_data[0]);
            println!("{:?}", field_data[1]);
            println!("{:?}", field_data[2]);
            assert_eq!(field_data[0], PacketFieldValue::Number(1.0));
            assert_eq!(field_data[1], PacketFieldValue::Number(2.0));
            assert_eq!(field_data[2], PacketFieldValue::Number(3.0));
        }
    }
    //test for parsing negative numbers, succeeds as long as the packet structure marks that the data field is for signed values
    #[test]
    fn test_nonpositive_parsing() {
        let packet_structure_manager = PacketStructureManager::default();
        let manager_arc = Arc::new(Mutex::new(packet_structure_manager));
        let mut csv_read_driver = CSVReadDriver::new(manager_arc);
        let mut result = csv_read_driver.init_device("src/test_files/test2.csv", 0);
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        for packet in packet_vector {
            let field_data = &packet.field_data;
            assert_eq!(field_data[0], PacketFieldValue::Number(9.0));
            assert_eq!(field_data[1], PacketFieldValue::Number(-8.0));
            assert_eq!(field_data[2], PacketFieldValue::Number(47.0));
            assert_eq!(field_data[3], PacketFieldValue::Number(0.0));
            assert_eq!(field_data[4], PacketFieldValue::Number(-25.0));
        }
    }
    // if a row has fewer data fields than declared by the CSV header, parsing should fail
    #[test]
    fn test_parsing_missing_columns_in_row() {
        let packet_structure_manager = PacketStructureManager::default();
        let manager_arc = Arc::new(Mutex::new(packet_structure_manager));
        let mut csv_read_driver = CSVReadDriver::new(manager_arc);
        let mut result = csv_read_driver.init_device("src/test_files/test_missing_field.csv", 0);
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_err());
        assert!(packet_vector.is_empty());
    }
    //parse two rows from the same file
    #[test]
    fn test_parsing_two_rows() {
        let packet_structure_manager = PacketStructureManager::default();
        let manager_arc = Arc::new(Mutex::new(packet_structure_manager));
        let mut csv_read_driver = CSVReadDriver::new(manager_arc);
        let mut result = csv_read_driver.init_device("src/test_files/test5.csv", 0);
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        {
            let packet = &packet_vector[0];
            let field_data = &packet.field_data;
            assert_eq!(field_data.len(), 5);
            assert_eq!(field_data[0], PacketFieldValue::Number(9.0));
            assert_eq!(field_data[1], PacketFieldValue::Number(-8.0));
            assert_eq!(field_data[2], PacketFieldValue::Number(47.0));
            assert_eq!(field_data[3], PacketFieldValue::Number(0.0));
            assert_eq!(field_data[4], PacketFieldValue::Number(-25.0));
        }
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        let packet = &packet_vector[1];
        let field_data = &packet.field_data;
        assert_eq!(field_data[0], PacketFieldValue::Number(1.0));
        assert_eq!(field_data[1], PacketFieldValue::Number(-2.0));
        assert_eq!(field_data[2], PacketFieldValue::Number(0.0));
        assert_eq!(field_data[3], PacketFieldValue::Number(4.0));
        assert_eq!(field_data[4], PacketFieldValue::Number(0.0));
    }
    //parse big numbers
    #[test]
    fn test_big_num_parsing() {
        let packet_structure_manager = PacketStructureManager::default();
        let manager_arc = Arc::new(Mutex::new(packet_structure_manager));
        let mut csv_read_driver = CSVReadDriver::new(manager_arc);
        let mut result = csv_read_driver.init_device("src/test_files/test3.csv", 0);
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        for packet in packet_vector {
            let field_data = &packet.field_data;
            assert_eq!(field_data[0], PacketFieldValue::Number(65.0));
            assert_eq!(field_data[1], PacketFieldValue::Number(129.0));
            assert_eq!(field_data[2], PacketFieldValue::Number(257.0));
            assert_eq!(field_data[3], PacketFieldValue::Number(529.0));
            assert_eq!(field_data[4], PacketFieldValue::Number(1000000000.0));
        }
    }
    //parse decimals
    #[test]
    fn test_decimal_parsing() {
        let packet_structure_manager = PacketStructureManager::default();
        let manager_arc = Arc::new(Mutex::new(packet_structure_manager));
        let mut csv_read_driver = CSVReadDriver::new(manager_arc);
        let mut result = csv_read_driver.init_device("src/test_files/test4.csv", 0);
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        for packet in packet_vector {
            let field_data = &packet.field_data;
            assert_eq!(field_data[0], PacketFieldValue::Number(5.0));
            assert_eq!(field_data[1], PacketFieldValue::Number(0.1));
            assert_eq!(field_data[2], PacketFieldValue::Number(0.27));
            assert_eq!(field_data[3], PacketFieldValue::Number(3.141592));
            assert_eq!(field_data[4], PacketFieldValue::Number(239.52));
        }
    }

    #[test]
    fn test_parsing_three_rows() {
        let packet_structure_manager = PacketStructureManager::default();
        let manager_arc = Arc::new(Mutex::new(packet_structure_manager));
        let mut csv_read_driver = CSVReadDriver::new(manager_arc);
        let mut result = csv_read_driver.init_device("src/test_files/test5.csv", 0);
        assert!(result.is_ok());
        let packet_vector = &mut vec![];
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        {
            let packet = &packet_vector[0];
            let field_data = &packet.field_data;
            assert_eq!(field_data.len(), 5);
            assert_eq!(field_data[0], PacketFieldValue::Number(9.0));
            assert_eq!(field_data[1], PacketFieldValue::Number(-8.0));
            assert_eq!(field_data[2], PacketFieldValue::Number(47.0));
            assert_eq!(field_data[3], PacketFieldValue::Number(0.0));
            assert_eq!(field_data[4], PacketFieldValue::Number(-25.0));
        }
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        {
            let packet = &packet_vector[1];
            let field_data = &packet.field_data;
            assert_eq!(field_data[0], PacketFieldValue::Number(1.0));
            assert_eq!(field_data[1], PacketFieldValue::Number(-2.0));
            assert_eq!(field_data[2], PacketFieldValue::Number(0.0));
            assert_eq!(field_data[3], PacketFieldValue::Number(4.0));
            assert_eq!(field_data[4], PacketFieldValue::Number(0.0));
        }
        result = csv_read_driver.parse_device_data(&mut vec![], packet_vector);
        assert!(result.is_ok());
        {
            let packet = &packet_vector[2];
            let field_data = &packet.field_data;
            assert_eq!(field_data[0], PacketFieldValue::Number(5.0));
            assert_eq!(field_data[1], PacketFieldValue::Number(-5.0));
            assert_eq!(field_data[2], PacketFieldValue::Number(5.0));
            assert_eq!(field_data[3], PacketFieldValue::Number(-5.0));
            assert_eq!(field_data[4], PacketFieldValue::Number(5.0));
        }
    }
}
