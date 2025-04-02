use std::{str::from_utf8, sync::{Arc, Mutex}};

use anyhow::bail;

use crate::{communication_manager::CommsIF, models::packet::Packet, packet_structure_manager::PacketStructureManager};

use super::altos_packet_parser::AltosPacketParser;
const PRINT_PARSING: bool = false;

#[derive(Default)]
pub struct TeleDongleDriver {
    port: Option<Box<dyn serialport::SerialPort>>,
    packet_parser: AltosPacketParser,
    baud: u32,
    id: usize,
    packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
}

impl CommsIF for TeleDongleDriver {

    /// Set the path of the active port
    /// If path is empty, any active port is closed
    /// 
    /// # Errors
    /// 
    /// Returns an error if port_name is invalid, or if unable to clear the device buffer
    fn init_device(&mut self, port_name: &str, _baud: u32, ps_manager: Arc<Mutex<PacketStructureManager>>) -> anyhow::Result<()> {
        self.packet_structure_manager = ps_manager;
        if port_name.is_empty() {
            self.port = None;
        } else {
            self.baud = 9600;
            let mut new_port = serialport::new(port_name, self.baud).flow_control(serialport::FlowControl::None).open()?;
            // Short non-zero timeout is needed to receive data from the serialport when
            // the buffer isn't full yet.
            new_port.set_timeout(std::time::Duration::from_millis(1))?;
            self.port = Some(new_port);

            //setup commands for the radio
            self.write_port(&vec![0x7E, 0x0A, 0x45,0x20,0x30,0x0A,0x6D,0x20,0x30,0x0A])?;
            self.get_device_packets(&mut vec![])?;
            self.write_port(&vec![0x6D,0x20,0x32,0x30,0x0A,0x6D,0x20,0x30,0x0A,0x63,0x20,0x73,0x0A,0x66,0x0A,0x76,0x0A])?;
            self.get_device_packets(&mut vec![])?;
            self.write_port(&vec![0x6D,0x20,0x32,0x30,0x0A,0x6D,0x20,0x30,0x0A,0x63,0x20,0x46,0x20,0x34,0x33,0x34,0x35,0x35,0x30,0x0A,0x6D,0x20,0x32,0x30,0x0A,0x6D,0x20,0x30,0x0A,0x6D,0x20,0x32,0x30,0x0A,0x6D,0x20,0x30,0x0A,0x63,0x20,0x54,0x20,0x30,0x0A,0x6D,0x20,0x32,0x30,0x0A,0x6D,0x20,0x32,0x30,0x0A])?;
        }
        Ok(())
    }

    /// Returns true if there is an active port
    fn is_init(&mut self) -> bool {
        self.port.is_some()
    }

    /// Reads bytes from the active port and adds new bytes to the write_buffer
    /// returns an empty set when the function runs successfully, 
    /// 
    /// # Errors
    /// bails and returns an error if there is no active port
    fn get_device_packets(&mut self, write_buffer: &mut Vec<Packet>) -> anyhow::Result<()> {
        let active_port = match self.port.as_mut() {
            Some(port) => port,
            None => bail!("No read port has been set")
        };

        let mut buffer = [0; 4096];
        let _bytes_read = active_port.read(&mut buffer)?;
        let str = from_utf8(&buffer)?;
        let mut parsed_str = "".to_owned();
        for c in str.chars() {
            if c.is_ascii_hexdigit() {
                parsed_str.push(c);
            }
        }
        let decoded = hex::decode(parsed_str)?;
        
        // Clone to a vec so we can return it easily, especially as we don't
        // know how large it will end up being at compile time.
        self.packet_parser.push_data(&decoded, PRINT_PARSING);
        write_buffer.extend_from_slice(&self.packet_parser.parse_packets(&self.packet_structure_manager.lock().unwrap(), PRINT_PARSING)?); 
        Ok(())
    }

    /// Attempt to write bytes to the radio test port
    /// 
    /// # Errors
    /// 
    /// returns an error if there is no active port
    fn write_port(&mut self, packet: &[u8]) -> anyhow::Result<()> {
        let port = match self.port.as_mut() {
            Some(port) => port,
            None => bail!("No active test port")
        };

        port.write(packet)?;

        Ok(())
    }

    fn set_id(&mut self, id: usize){
        self.id = id;
    }
    fn get_id(&self) -> usize {
        return self.id;
    }
    
    fn get_type(&self) -> String {
        return "TeleDongle".to_owned();
    }
    
    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()> {
        let active_port = match self.port.as_mut() {
            Some(port) => port,
            None => bail!("No read port has been set")
        };

        let mut buffer = [0; 4096];
        let _bytes_read = active_port.read(&mut buffer)?;
        let str = from_utf8(&buffer)?;
        let mut parsed_str = "".to_owned();
        for c in str.chars() {
            if c.is_ascii_hexdigit(){
                parsed_str.push(c);
            }
        }
        data_vector.append(&mut hex::decode(parsed_str)?);
        return Ok(());
    }
    
    fn parse_device_data(&mut self, data_vector: &mut Vec<u8>, packet_vector: &mut Vec<Packet>) -> anyhow::Result<()> {
        self.packet_parser.push_data(&data_vector, PRINT_PARSING);
        packet_vector.extend_from_slice(&self.packet_parser.parse_packets(&self.packet_structure_manager.lock().unwrap(), PRINT_PARSING)?); 
        return Ok(());
    }
}
