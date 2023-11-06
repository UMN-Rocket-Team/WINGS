use anyhow::bail;
use serde::Serialize;
use libftd2xx::{Ftdi, FtdiCommon};
use tauri::Manager;

const BAUD_RATE: u32 = 57600;

#[derive(PartialEq, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SerialPortNames {
    name: String,
    manufacturer_name: Option<String>,
    product_name: Option<String>,
}

#[derive(Default)]
pub struct SerialManager {
    previous_available_ports: Vec<SerialPortNames>,
    active_port: Option<Box<dyn serialport::SerialPort>>,
    test_port: Option<Box<dyn serialport::SerialPort>>
}

impl SerialManager {
    /// Returns a list of all accessible serial ports
    pub fn get_available_ports(&self) -> Result<Vec<SerialPortNames>, serialport::Error> {
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
                        Some(SerialPortNames {
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

    /// Return Some() if the ports have changed since the last call, otherwise None if they are the same.
    pub fn get_new_available_ports(&mut self) -> Option<Vec<SerialPortNames>> {
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
    pub fn set_active_port(&mut self, port_name: &str) -> anyhow::Result<()> {
        if port_name.is_empty() {
            self.active_port = None;
        } else {
            let mut port = serialport::new(port_name, BAUD_RATE).open()?;
            port.clear(serialport::ClearBuffer::All)?;
            // Short non-zero timeout is needed to receive data from the serialport when
            // the buffer isn't full yet.
            port.set_timeout(std::time::Duration::from_millis(1))?;
            self.active_port = Some(port);
        }
        Ok(())
    }

    /// Returns true if there is an active port
    pub fn has_active_port(&self) -> bool {
        return self.active_port.is_some()
    }

    /// Read bytes from the active port
    pub fn read_active_port(&mut self) -> anyhow::Result<Vec<u8>> {
        let active_port = match self.active_port.as_mut() {
            Some(port) => port,
            None => bail!("No active port")
        };

        let mut buffer = [0; 1024];
        let bytes_read = active_port.read(&mut buffer)?;

        // Clone to a vec so we can return it easily, especially as we don't
        // know how large it will end up being at compile time.
        let output = buffer[..bytes_read].to_vec();
        Ok(output)
    }

    /// Set the path of the test radio port
    /// If path is empty, any existing port is closed
    pub fn set_test_port(&mut self, port_name: &str) -> anyhow::Result<()> {
        if port_name.is_empty() {
            self.test_port = None;
        } else {
            let port = serialport::new(port_name, BAUD_RATE).open()?;
            port.clear(serialport::ClearBuffer::All)?;
            self.test_port = Some(port);
        }
        Ok(())
    }

    /// Attempt to write bytes to the radio test port
    pub fn write_test_port(&mut self, packet: &[u8]) -> anyhow::Result<()> {
        let test_port = match self.test_port.as_mut() {
            Some(port) => port,
            None => bail!("No active test port")
        };

        test_port.write(packet)?;

        Ok(())
    }
}
