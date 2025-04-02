use std::sync::Arc;

use anyhow::bail;
use tauri::AppHandle;

use crate::{communication_manager::{CommsIF, DeviceName}, models::packet::Packet, packet_parser::SerialPacketParser, packet_structure_manager::PacketStructureManager};

const PRINT_PARSING: bool = false;
#[derive(Default)]
pub struct SerialPortDriver {
    previous_available_ports: Vec<DeviceName>,
    port: Option<Box<dyn serialport::SerialPort>>,
    packet_parser: SerialPacketParser,
    baud: u32,
    id: usize,
    app_handle: Option<AppHandle>,
    packet_structure_manager: Arc<PacketStructureManager>,
}

impl CommsIF for SerialPortDriver{
    
    /// Attempts to set the port for comms with the rfd driver
    /// 
    /// # Errors
    /// 
    /// Returns an error if port_name is invalid, or if unable to clear the device buffer
    fn init_device(&mut self, port_name: &str , baud: u32, ps_manager: Arc<PacketStructureManager>)  -> anyhow::Result<()> {
        self.packet_structure_manager = ps_manager.clone();
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
    fn write_port(&mut self, packet: &[u8])  -> anyhow::Result<()> {
        let test_port = match self.port.as_mut() {
            Some(some_port) => some_port,
            None => bail!("No active test port")
        };
        
        test_port.write(packet)?;
        Ok(())
    }

    /// Reads bytes from the active port and adds new bytes to the write_buffer
    /// 
    /// # Errors
    /// 
    /// bails and returns an error if there is no active port
    fn get_device_packets(&mut self, write_buffer: &mut Vec<Packet>) -> anyhow::Result<()> {
        let active_port = match self.port.as_mut() {
            Some(port) => port,
            None => bail!("No read port has been set")
        };

        let mut buffer = [0; 4096];
        let bytes_read = active_port.read(&mut buffer)?;

        self.packet_parser.push_data(&buffer[..bytes_read], PRINT_PARSING);
        write_buffer.extend_from_slice(&self.packet_parser.parse_packets(&self.packet_structure_manager, PRINT_PARSING)); 
        Ok(())
    }

    /// Return Some() if the ports have changed since the last call, otherwise None if they are the same.
    fn get_new_available_ports(&mut self) -> Option<Vec<DeviceName>> {
        match self.get_available_ports() {
            Ok(new_ports) => {
                if new_ports == self.previous_available_ports {
                    None
                } else {
                    self.previous_available_ports = new_ports.clone();
                    Some(new_ports)
                }
            },
            Err(_) => None
        }
    }

    /// Returns true if there is an active port
    fn is_init(&mut self) -> bool {
        self.port.is_some()
    }
    fn set_id(&mut self, id: usize){
        self.id = id;
    }
    fn get_id(&self) -> usize {
        return self.id;
    }
    
    fn get_type(&self) -> String {
        return "SerialPort".to_owned();
    }
    
    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()> {
        let active_port = match self.port.as_mut() {
            Some(port) => port,
            None => bail!("No read port has been set")
        };

        let mut buffer = [0; 4096];
        let bytes_read = active_port.read(&mut buffer)?;
        data_vector.extend_from_slice(&buffer[..bytes_read]);
        return Ok(());
    }
    
    fn parse_device_data(&mut self, data_vector: &mut Vec<u8>, packet_vector: &mut Vec<Packet>) -> anyhow::Result<()> {
        self.packet_parser.push_data(data_vector, PRINT_PARSING);
        packet_vector.extend_from_slice(&self.packet_parser.parse_packets(&self.packet_structure_manager, PRINT_PARSING)); 
        return Ok(());
    }
}
impl SerialPortDriver {

    /// Returns a list of all accessible serial ports
    /// 
    /// # Errors
    /// 
    /// Returns an error if no ports were successfully found, 
    pub fn get_available_ports(&self) -> Result<Vec<DeviceName>, serialport::Error> {
        let ports = serialport::available_ports()?
            .into_iter()
            .filter_map(|port| match port.port_type {
                serialport::SerialPortType::UsbPort(usb_info) => {
                    // On macOS, each serial port shows up as both eg.:
                    //  - /dev/cu.usbserial-AK06O4AO
                    //  - /dev/tty.usbserial-AK06O4AO
                    // For our use, these are equivalent, so we'll filter one out to avoid confusion.
                    if cfg!(target_os = "macos") && port.port_name.starts_with("/dev/cu.usbserial-") {
                        None
                    } else {
                        Some(DeviceName {
                            name: port.port_name,
                            manufacturer_name: usb_info.manufacturer,
                            product_name: usb_info.product,
                        })
                    }
                },
                serialport::SerialPortType::PciPort
                | serialport::SerialPortType::BluetoothPort
                | serialport::SerialPortType::Unknown => None,
            })
            .collect();
        Ok(ports)
    }

}