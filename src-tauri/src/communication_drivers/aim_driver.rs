use std::{ffi::CString, sync::{Arc, Mutex}, thread::sleep, time::Duration};

use anyhow::bail;
use hidapi::{HidApi, HidDevice};

use crate::{communication_manager::CommsIF, models::packet::Packet, packet_structure_manager::PacketStructureManager};

use super::aim_parser::AimParser;
pub struct AimDriver {
    device: Option<HidDevice>,
    packet_parser: AimParser,
    baud: u32,
    id: usize,
    last_read: [u8;64] 
}

impl CommsIF for AimDriver{
    
     ///creates a new instance of a comms device with the given packet structure manager
     fn new(
        packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
    ) -> Self 
    where
        Self: Sized {
        let parser = AimParser::default(&mut packet_structure_manager.lock().expect("ps_manager_poisoned").clone());
        return AimDriver{
            device: None,
            packet_parser: parser,
            baud: 0,
            id: 0,
            last_read: [0;64],
        }
    }

    /// used to connect the object with a specific device
    fn init_device(&mut self, port_name: &str , baud: u32)  -> anyhow::Result<()> {
        if port_name.is_empty() {
            self.device = None;
        } else {
            self.baud = baud;
            let hid_api = HidApi::new()?;
            self.device = hid_api.open_path(CString::new(port_name)?.as_c_str()).ok();

            let mut output: [u8;64] = [0;64];
            let mut input: [u8;64] = [0;64];
            output[0] = 3;
            output[1] = 3;

            match &self.device {
                Some(base_station) => {
                    let _ = base_station.write(&mut output);
                    sleep(Duration::from_millis(100));
                    let result = base_station.read(&mut input);
                    match result{
                        Ok(_) => {},
                        Err(error) => {
                            bail!(anyhow::anyhow!(error).context("failed to connect to entacore"))
                        },
                    }
                },
                None => bail!("no device stored within output of HID API"),
            }
        }
        Ok(())
    }

    /// Attempt to write bytes to the radio test port
    /// 
    /// # Errors
    /// 
    /// returns an error if the device isn't initialized 
    fn write_port(&mut self, _: &[u8])  -> anyhow::Result<()> {
        Err(anyhow::anyhow!("Wings does not currently support sending packets to an aim xtra"))
    }

    fn get_device_packets(&mut self, _: &mut Vec<Packet>) -> anyhow::Result<()> {
        todo!()
    }

    /// Returns true if there is an active port
    fn is_init(&self) -> bool {
        self.device.is_some()
    }
    fn set_id(&mut self, id: usize){
        self.id = id;
    }
    fn get_id(&self) -> usize {
        return self.id;
    }
    
    fn get_type(&self) -> String {
        return "AimXtra".to_owned();
    }
    
    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()> {
        // let active_port = match self.device.as_mut() {
        //     Some(port) => port,
        //     None => bail!("No read port has been set")
        // };

        // let mut buffer = [0; 4096];
        // let bytes_read = active_port.read(&mut buffer)?;
        // data_vector.extend_from_slice(&buffer[..bytes_read]);
        match &self.device {
            Some(base_station) => {
                let mut output: [u8;64] = [0;64];
                let mut input: [u8;64] = [0;64];
                output[0] = 0x03;
                output[1] = 0x12;
                let _ = base_station.write(&mut output);
                let result = base_station.read_timeout(&mut input, 10);
                match result{
                    Ok(_) => {
                        if self.last_read != input{
                            data_vector.extend_from_slice(&input);
                            self.last_read = input;
                        }
                    },
                    Err(_) => {
                        //doing nothing because we didn't read a packet
                    }
                }
                sleep(Duration::from_secs(1));
            }
            None => bail!("not initialized"),
        }
        return Ok(());
    }
    
    fn parse_device_data(&mut self, data_vector: &mut Vec<u8>, packet_vector: &mut Vec<Packet>) -> anyhow::Result<()> {
        return self.packet_parser.parse_transmission(data_vector,packet_vector);
    }
}