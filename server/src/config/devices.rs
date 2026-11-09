use serialport::{SerialPort, SerialPortBuilder, SerialPortInfo, SerialPortType};
use wings_packet_parser::communication_manager::DeviceName;

use crate::config::RadioOptions;

pub struct SerialDevice {
    port: Box<dyn serialport::SerialPort>,
    role: SerialDeviceRole,
    baud: u32
}

impl SerialDevice {
    pub fn baud_rate(&self) -> u32 {
        self.baud
    }
    pub fn port(&self) -> &Box<dyn serialport::SerialPort> {
        &self.port
    }
    pub fn role(&self) -> SerialDeviceRole {
        self.role
    }

    pub fn new(port: Box<dyn SerialPort>, role: SerialDeviceRole, baud: u32) -> Self {
        Self {
            port,
            role,
            baud
        }
    }
}


#[derive(Default, Copy, Clone)]
pub enum SerialDeviceRole {
    #[default]
    Transmitter,
    Reciever
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


pub fn match_radio_devices(
    config: &RadioOptions,
) -> Result<Vec<SerialDevice>, serialport::Error> {
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
            let device = construct_device_name(device)?;
            // do some matching w/ config file
            if let Some(transmitter) = config.transmitter()
                && device == *transmitter.device()
            {
                {
                    let dev = serialport::new(transmitter.device().value.clone(), transmitter.baud_rate()).open()?;
                    matched_devices.push(SerialDevice::new(dev,SerialDeviceRole::Transmitter,transmitter.baud_rate()))
                }
            }
            if let Some(receiver) = config.receiver()
                && device == *receiver.device()
            {
                {
                    let dev = serialport::new(receiver.device().value.clone(), receiver.baud_rate()).open()?;
                    matched_devices.push(SerialDevice::new(dev,SerialDeviceRole::Reciever,receiver.baud_rate()))
                }
            }
        }
    }

    Ok(matched_devices) // returns a list of our selected devices in our config file
}
