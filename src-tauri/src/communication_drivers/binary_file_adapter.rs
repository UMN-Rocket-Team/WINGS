// ****
// Written by Kuba K
// Communications device driver for reading byte files and Putty outputs
//
// ****
use crate::{
    communication_manager::CommsIF,
    models::packet::Packet,
    packet_structure_manager::PacketStructureManager, state::{mutex_utils::use_state_in_mutex},
};
use anyhow::{bail, Context};
use std::{fs::File, io::Read, sync::{Arc, Mutex}};

use super::serial_packet_parser::SerialPacketParser;

const PRINT_PARSING: bool = false;

#[derive(Default)]
/// The `ByteReadDriver` is an implementation of the `CommsIF` communications interface. 
/// it reads from a binary file as if the file was a serial port. this is useful for replaying .wings files or PuTTY outputs
/// 
/// Properties:
/// 
/// * `file`: A Handle of the file that is being used as a data source
/// * `id`: a device id mandated by the `CommsIF``
/// * `packet_parser`: A packet parser that will be used to process packets from the binary
/// * `packet_structure_manager`: A reference to a PacketStructureManager that defines all the packets the communications driver will be working with
pub struct BinaryFileAdapter {
    file: Option<File>,
    id: usize,
    packet_parser: SerialPacketParser,
    packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
}
impl CommsIF for BinaryFileAdapter {

     ///creates a new instance of a comms device with the given packet structure manager
     fn new(
        packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
    ) -> Self 
    where
        Self: Sized {
        return BinaryFileAdapter{
            file: None,
            packet_parser: Default::default(),
            id: 0,
            packet_structure_manager: packet_structure_manager
        }
    }

    fn init_device(
        &mut self,
        file_name: &str,
        _baud: u32,
    ) -> anyhow::Result<()> {
        match File::open(file_name) {
            Ok(new_file) => {
                self.file = Some(new_file);
                Ok(())
            }
            Err(err) => bail!(err),
        }
    }

    //This file should never have bytes written to it by wings. look at file_handling.rs to see how we write data
    fn write_port(&mut self, packet: &[u8]) -> anyhow::Result<()> {
        let _ = packet;
        Ok(())
    }

    fn get_device_packets(&mut self, write_buffer: &mut Vec<Packet>) -> anyhow::Result<()> {
        if self.file.is_some() {
            let mut buffer = [0; 4096];
            match self
                .file
                .as_mut()
                .context("failed to load file")?
                .read(&mut buffer)
            {
                Ok(_) => {
                    if buffer == [0; 4096] {
                        return Ok(());
                    }
                    self.packet_parser.push_data(&buffer, PRINT_PARSING);
                    use_state_in_mutex(&self.packet_structure_manager, &mut |ps_manager: &mut PacketStructureManager| -> anyhow::Result<()>{
                        write_buffer.extend_from_slice(
                            &self
                                .packet_parser
                                .parse_packets(ps_manager, PRINT_PARSING)?
                        );
                        Ok(())
                    }).expect("poison!");
                    Ok(())
                }
                Err(err) => bail!(err),
            }
        } else {
            bail!("reading from uninitialized driver");
        }
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
        return "ByteFile".to_owned();
    }

    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()> {
        let mut buffer: [u8; 4096] = [0; 4096];
        self.file
            .as_mut()
            .context("failed to load file")?
            .read(&mut buffer)?; //question mark operator returns error if we fail
        data_vector.append(&mut buffer.to_vec());
        return Ok(()); // returns ok if everything succeeded
    }

    fn parse_device_data(
        &mut self,
        data_vector: &mut Vec<u8>,
        packet_vector: &mut Vec<Packet>,
    ) -> anyhow::Result<()> {
        self.packet_parser.push_data(data_vector, PRINT_PARSING);
        return use_state_in_mutex(&self.packet_structure_manager, &mut|ps_manager| -> anyhow::Result<()>{
            packet_vector.extend_from_slice(
            &self
                .packet_parser
                .parse_packets(ps_manager, PRINT_PARSING)?
            );
            return Ok(());
        }).expect("poison!");
    }
}
