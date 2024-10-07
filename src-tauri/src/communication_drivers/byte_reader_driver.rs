// ****
// Written by Kuba K
// Communications device driver for reading byte files and Putty outputs
// 
// ****
use std::{fs::File, io::Read};
use anyhow::{bail, Context};
use tauri::{AppHandle, Manager};
use crate::{communication_manager::{CommsIF, DeviceName}, file_handling::config_struct::ConfigStruct, models::packet::Packet, packet_parser::SerialPacketParser, state::generic_state::{get_clone, ConfigState}};

const PRINT_PARSING: bool = false;

#[derive(Default)]
pub struct ByteReadDriver {
    file: Option<File>,
    id: usize,
    packet_parser: SerialPacketParser,
    config: ConfigStruct

}
impl CommsIF for ByteReadDriver{
    fn init_device(&mut self, file_name: &str, _baud: u32, app_handle: AppHandle)  -> anyhow::Result<()> {
        self.config = get_clone(&app_handle.state::<ConfigState>())?;
        match File::open(file_name){
            Ok(new_file) => {
                self.file = Some(new_file); 
                Ok(())
            },
            Err(err) => bail!(err),
        }
    }

    //This file should never have bytes written to it by wings. look at file_handling.rs to see how we write data
    fn write_port(&mut self, packet: &[u8])  -> anyhow::Result<()> {
        let _ = packet;
        Ok(())
    }

    fn get_device_packets(&mut self, write_buffer: &mut Vec<Packet>) -> anyhow::Result<()> {
        if self.is_init(){
            let mut buffer = [0; 4096];
            match self.file.as_mut().context("failed to load file")?.read(&mut buffer) {
                Ok(_) => {
                    if buffer == [0; 4096] {
                        return Ok(());
                    }
                    self.packet_parser.push_data(&buffer, PRINT_PARSING);
                    write_buffer.extend_from_slice(&self.packet_parser.parse_packets(&self.config.packet_structure_manager, PRINT_PARSING)); 
                    Ok(())
                },
                Err(err) =>  bail!(err),
            }
        }
        else{
            bail!("reading from uninitialized driver");
        }
    }

    //Picking files to read is done by the frontend. We don't need to worry about scanning for files that the user might want
    fn get_new_available_ports(&mut self) -> std::option::Option<Vec<DeviceName>> {
        return None;
    }

    fn is_init(&mut self) -> bool {
        self.file.is_some()
    }
    fn set_id(&mut self, id: usize){
        self.id = id;
    }
    fn get_id(&self) -> usize {
        return self.id;
    }

    fn get_type(&self) -> String {
        return "ByteFile".to_owned();
    }
    
    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()> {
        self.file.as_mut().context("failed to load file")?.read(data_vector)?;//question mark operator returns error if we fail
        return Ok(());// returns ok if everything succeeded
    }
    
    fn parse_device_data(&mut self, data_vector: &mut Vec<u8>, packet_vector: &mut Vec<Packet>) -> anyhow::Result<()> {
        self.packet_parser.push_data(data_vector, PRINT_PARSING);
        packet_vector.extend_from_slice(&self.packet_parser.parse_packets(&self.config.packet_structure_manager, PRINT_PARSING));
        return Ok(());
    }
}