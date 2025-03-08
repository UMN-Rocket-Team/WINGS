// ****
// Written by Kuba K
// Communications device driver for reading byte files and Putty outputs
//
// ****
use crate::{
    communication_manager::CommsIF,
    models::packet::Packet,
    packet_structure_manager::PacketStructureManager,
};
use anyhow::{bail, Context};
use std::{fs::File, io::Read, sync::Arc};

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
pub struct ByteReadDriver {
    file: Option<File>,
    id: usize,
    packet_parser: SerialPacketParser,
    packet_structure_manager: Arc<PacketStructureManager>,
}
impl CommsIF for ByteReadDriver {
    fn init_device(
        &mut self,
        file_name: &str,
        _baud: u32,
        ps_manager: Arc<PacketStructureManager>,
    ) -> anyhow::Result<()> {
        self.packet_structure_manager = ps_manager;
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
        if self.is_init() {
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
                    write_buffer.extend_from_slice(
                        &self
                            .packet_parser
                            .parse_packets(&self.packet_structure_manager, PRINT_PARSING)?,
                    );
                    Ok(())
                }
                Err(err) => bail!(err),
            }
        } else {
            bail!("reading from uninitialized driver");
        }
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
        return "ByteFile".to_owned();
    }

    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()> {
        self.file
            .as_mut()
            .context("failed to load file")?
            .read(data_vector)?; //question mark operator returns error if we fail
        return Ok(()); // returns ok if everything succeeded
    }

    fn parse_device_data(
        &mut self,
        data_vector: &mut Vec<u8>,
        packet_vector: &mut Vec<Packet>,
    ) -> anyhow::Result<()> {
        self.packet_parser.push_data(data_vector, PRINT_PARSING);
        packet_vector.extend_from_slice(
            &self
                .packet_parser
                .parse_packets(&self.packet_structure_manager, PRINT_PARSING)?,
        );
        return Ok(());
    }
}
