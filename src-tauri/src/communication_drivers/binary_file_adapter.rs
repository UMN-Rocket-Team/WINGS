// ****
// Written by Kuba K
// Communications device driver for reading byte files and Putty outputs
//
// ****
use crate::{
    communication_manager::CommsIF, models::packet_parser::PacketParser,
    packet_structure_manager::PacketStructureManager, state::mutex_utils::use_state_in_mutex,
};
use anyhow::{bail, Context};
use std::{
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
};

use super::midwest_adapter::register_midwest_packet_structures;

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
    packet_parser: Option<Box<dyn PacketParser>>,
    packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
}
impl CommsIF for BinaryFileAdapter {
    ///creates a new instance of a comms device with the given packet structure manager
    fn new(
        packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
        packet_parser: Option<impl PacketParser + 'static>,
    ) -> Self
    where
        Self: Sized,
    {
        // register midwest packet structures for binary files from Midwest flights
        use_state_in_mutex(&packet_structure_manager, &mut |ps_ref| {
            if let Err(err) = register_midwest_packet_structures(ps_ref) {
                eprintln!("Failed to register Midwest packet structures: {err}");
            }
        });
        // Some(Box::new(SerialPacketParser::default())) as Box<dyn PacketParser>)
        BinaryFileAdapter {
            file: None,
            packet_parser: Some(Box::new(packet_parser.unwrap())),
            id: 0,
            packet_structure_manager,
        }
    }

    fn init_device(&mut self, file_name: &str, _baud: u32) -> anyhow::Result<()> {
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

    fn is_init(&self) -> bool {
        self.file.is_some()
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_type(&self) -> String {
        "ByteFile".to_owned()
    }

    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()> {
        let mut buffer: [u8; 4096] = [0; 4096];
        let bytes_read = self
            .file
            .as_mut()
            .context("failed to load file")?
            .read(&mut buffer)?; //question mark operator returns error if we fail
        data_vector.extend_from_slice(&buffer[..bytes_read]);
        Ok(()) // returns ok if everything succeeded
    }

    fn get_parser(&mut self) -> Option<&mut (dyn PacketParser + 'static)> {
        self.packet_parser.as_deref_mut()
    }

    fn get_packet_structure_manager(&self) -> Arc<Mutex<PacketStructureManager>> {
        self.packet_structure_manager.clone()
    }
}
