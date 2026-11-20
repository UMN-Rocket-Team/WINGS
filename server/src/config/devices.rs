use serialport::{SerialPort, SerialPortInfo, SerialPortType};
use tracing::info;
use wings_packet_parser::communication_manager::DeviceName;

use crate::config::{Radio, RadioRole};

#[derive(Debug)]
pub struct SerialDevice {
    sn: String,
    port: Box<dyn serialport::SerialPort>,
    baud: u32,
    roles: u8,
}

impl SerialDevice {
    pub fn sn(&self) -> &str {
        &self.sn
    }
    pub fn baud_rate(&self) -> u32 {
        self.baud
    }
    pub fn port(&self) -> &Box<dyn serialport::SerialPort> {
        &self.port
    }

    pub fn new(sn: String, port: Box<dyn SerialPort>, baud: u32, roles: u8) -> Self {
        Self {
            sn,
            port,
            baud,
            roles,
        }
    }

    pub fn write(&mut self, packet: &[u8]) -> anyhow::Result<()> {
        self.port.write_all(packet)?;
        Ok(())
    }

    pub fn read(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()> {
        let mut buffer = [0; 4096];
        let bytes_read = self.port.read(&mut buffer)?;
        data_vector.extend_from_slice(&buffer[..bytes_read]);
        Ok(())
    }

    pub fn is_command(&self) -> bool {
        RadioRole::check_role(self.roles, RadioRole::Command)
    }

    pub fn is_transmitter(&self) -> bool {
        RadioRole::check_role(self.roles, RadioRole::Transmitter)
    }

    pub fn is_receiver(&self) -> bool {
        RadioRole::check_role(self.roles, RadioRole::Reciever)
    }
}

fn construct_device_name(device: SerialPortInfo) -> Result<DeviceName, serialport::Error> {
    match device.port_type {
        // create a Device Name for a usb port
        SerialPortType::UsbPort(usb) => {
            // DeviceName needs a serial number at the bare minimum
            let serial_number = match usb.serial_number {
                Some(s) => s,
                None => {
                    return Err(serialport::Error {
                        kind: serialport::ErrorKind::Unknown,
                        description: String::from("serial port serial_number does not exist"),
                    });
                }
            };

            Ok(DeviceName {
                name: serial_number,
                value: device.port_name,
                manufacturer_name: usb.manufacturer,
                product_name: usb.product,
            })
        }
        // any other type of device throws an error
        _ => Err(serialport::Error {
            kind: serialport::ErrorKind::NoDevice,
            description: format!("failed to match device of type {:?}", device.port_type),
        }),
    }
}

pub fn match_radio_devices(radios: &Vec<Radio>) -> Result<Vec<SerialDevice>, serialport::Error> {
    let mut matched_devices = vec![];
    let devices = serialport::available_ports();

    if devices.is_err() {
        return Err(serialport::Error {
            kind: serialport::ErrorKind::NoDevice,
            description: String::from("unable to list available ports"),
        });
    }

    // match against all devices
    for device in devices.unwrap() {
        {
            let device = match construct_device_name(device) {
                Ok(d) => d,
                Err(_) => continue,
            };
            info!("Device {:?}", device);
            // do some matching w/ config file
            for radio in radios {
                info!("Matched: {:?}", device);
                let serial_open = serialport::new(device.value.clone(), radio.baud_rate()).open();
                match serial_open {
                    Ok(dev) => matched_devices.push(SerialDevice::new(
                        device.name.clone(),
                        dev,
                        radio.baud_rate(),
                        radio.roles_mask(),
                    )),
                    Err(e) => {
                        println!("Error opening serial port: {}", e.to_string());
                    }
                }
            }
        }
    }

    Ok(matched_devices) // returns a list of our selected devices in our config file
}
