use std::str::from_utf8;

use anyhow::bail;
use tauri::{AppHandle, Manager};

use crate::{communication_manager::{CommsIF,DeviceName}, file_handling::config_struct::ConfigStruct, models::packet::Packet, packet_parser::SerialPacketParser, state::generic_state::{get_clone, ConfigState}};
const PRINT_PARSING: bool = false;

#[derive(Default)]
pub struct TeleDongleDriver {
    previous_available_ports: Vec<DeviceName>,
    port: Option<Box<dyn serialport::SerialPort>>,
    packet_parser: SerialPacketParser,
    baud: u32,
    id: usize,
    config: ConfigStruct
}
impl TeleDongleDriver {
    /// Returns a list of all accessible serial ports
    /// 
    /// # Errors
    /// 
    /// Returns an error if no ports were successfully found, 
    fn get_available_ports(&self) -> Result<Vec<DeviceName>, serialport::Error> {
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
impl CommsIF for TeleDongleDriver {

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

    /// Set the path of the active port
    /// If path is empty, any active port is closed
    /// 
    /// # Errors
    /// 
    /// Returns an error if port_name is invalid, or if unable to clear the device buffer
    fn init_device(&mut self, port_name: &str, _baud: u32, app_handle: AppHandle) -> anyhow::Result<()> {
        self.config = get_clone(&app_handle.state::<ConfigState>())?;
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
        write_buffer.extend_from_slice(&self.packet_parser.parse_packets(&self.config.packet_structure_manager, PRINT_PARSING)); 
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
        todo!()
    }
    
    fn parse_device_data(&mut self, data_vector: &mut Vec<u8>, packet_vector: &mut Vec<Packet>) -> anyhow::Result<()> {
        todo!()
    }
}
