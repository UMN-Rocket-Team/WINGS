use std::sync::{Arc, Mutex};

use anyhow::bail;

use crate::{
    communication_manager::CommsIF, models::packet::Packet,
    packet_structure_manager::PacketStructureManager, state::mutex_utils::use_state_in_mutex,
};

use super::serial_packet_parser::SerialPacketParser;

const PRINT_PARSING: bool = false;
#[derive(Default)]
pub struct SerialPortAdapter {
    port: Option<Box<dyn serialport::SerialPort>>,
    packet_parser: SerialPacketParser,
    baud: u32,
    id: usize,
    packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
}

impl CommsIF for SerialPortAdapter {
    ///creates a new instance of a comms device with the given packet structure manager
    fn new(packet_structure_manager: Arc<Mutex<PacketStructureManager>>) -> Self
    where
        Self: Sized,
    {
        SerialPortAdapter {
            port: None,
            packet_parser: Default::default(),
            baud: 0,
            id: 0,
            packet_structure_manager,
        }
    }

    /// Attempts to set the port for comms with the rfd driver
    ///
    /// # Errors
    ///
    /// Returns an error if port_name is invalid, or if unable to clear the device buffer
    fn init_device(&mut self, port_name: &str, baud: u32) -> anyhow::Result<()> {
        if port_name.is_empty() {
            self.port = None;
        } else {
            self.baud = baud;
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
    fn write_port(&mut self, packet: &[u8]) -> anyhow::Result<()> {
        let test_port = match self.port.as_mut() {
            Some(some_port) => some_port,
            None => bail!("No active test port"),
        };

        test_port.write_all(packet)?;
        Ok(())
    }

    /// Returns true if there is an active port
    fn is_init(&self) -> bool {
        self.port.is_some()
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_type(&self) -> String {
        "SerialPort".to_owned()
    }

    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()> {
        let active_port = match self.port.as_mut() {
            Some(port) => port,
            None => bail!("No read port has been set"),
        };

        let mut buffer = [0; 4096];
        let bytes_read = active_port.read(&mut buffer)?;
        data_vector.extend_from_slice(&buffer[..bytes_read]);
        Ok(())
    }

    fn parse_device_data(
        &mut self,
        data_vector: &mut Vec<u8>,
        packet_vector: &mut Vec<Packet>,
    ) -> anyhow::Result<()> {
        self.packet_parser.push_data(data_vector, PRINT_PARSING);
        use_state_in_mutex(
            &self.packet_structure_manager,
            &mut |parser| -> anyhow::Result<()> {
                packet_vector
                    .extend_from_slice(&self.packet_parser.parse_packets(parser, PRINT_PARSING)?);
                Ok(())
            },
        )?;
        Ok(())
    }
}
