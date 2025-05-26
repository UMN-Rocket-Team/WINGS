use std::sync::{Arc, Mutex};

use anyhow::bail;

use crate::{communication_manager::CommsIF, models::{packet::Packet}, packet_structure_manager::PacketStructureManager, state::mutex_utils::use_state_in_mutex};

use super::featherweight_parser;

#[derive(Default)]
pub struct FeatherweightAdapter {
    port: Option<Box<dyn serialport::SerialPort>>,
    baud: u32,
    id: usize,
    gps_packet_id: usize,
}

impl CommsIF for FeatherweightAdapter{
    
    ///creates a new instance of a comms device with the given packet structure manager
    fn new(
        packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
    ) -> Self 
    where
        Self: Sized {
        let id = use_state_in_mutex(&packet_structure_manager, &mut |ps_manager| {          
            ps_manager.enforce_packet_fields("FW GPS",vec![
                "TimeStamp",//Milliseconds
                "Altitude", //Feet
                "Lat",      //Degrees
                "Long",     //Degrees
                "Vel Lat",  //Feet per second
                "Vel Long", //Feet per second
                "Vel Vert"  //Feet per second
            ])
        });
        FeatherweightAdapter{
            port: None,
            baud: 115200,
            id: 0,
            gps_packet_id: id
        }
    }

    /// Attempts to set the port for comms with the rfd driver
    /// 
    /// # Errors
    /// 
    /// Returns an error if port_name is invalid, or if unable to clear the device buffer
    fn init_device(&mut self, port_name: &str , _baud: u32)  -> anyhow::Result<()> {
        println!("initial how");
        if port_name.is_empty() {
            self.port = None;
        } else {
            self.baud = 115200;
            let mut new_port = serialport::new(port_name, self.baud).open()?;
            new_port.clear(serialport::ClearBuffer::All)?;
            // Short non-zero timeout is needed to receive data from the serialport when
            // the buffer isn't full yet.
            new_port.set_timeout(std::time::Duration::from_millis(1))?;
            self.port = Some(new_port);
        }
        Ok(())
    }

    /// Attempt to write bytes to the radio test port
    /// 
    /// # Errors
    /// 
    /// returns an error if the device isn't initialized 
    fn write_port(&mut self, packet: &[u8])  -> anyhow::Result<()> {
        let test_port = match self.port.as_mut() {
            Some(some_port) => some_port,
            None => bail!("No active test port")
        };
        
        test_port.write_all(packet)?;
        Ok(())
    }

    /// Returns true if there is an active port
    fn is_init(&self) -> bool {
        self.port.is_some()
    }
    fn set_id(&mut self, id: usize){
        self.id = id;
    }
    fn get_id(&self) -> usize {
        self.id
    }
    
    fn get_type(&self) -> String {
        "FeatherWeight".to_owned()
    }
    
    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()> {
        let active_port = match self.port.as_mut() {
            Some(port) => port,
            None => bail!("No read port has been set")
        };

        let mut buffer = [0; 4096];
        let bytes_read = active_port.read(&mut buffer)?;
        
        data_vector.extend_from_slice(&buffer[..bytes_read]);
        Ok(())
    }
    
    fn parse_device_data(&mut self, data_vector: &mut Vec<u8>, packet_vector: &mut Vec<Packet>) -> anyhow::Result<()> {
        packet_vector.push(featherweight_parser::packet_from_byte_stream(data_vector,self.gps_packet_id)?);
        Ok(())
    }

}