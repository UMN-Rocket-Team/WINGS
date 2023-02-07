use anyhow::{bail, Result};
use serde::Serialize;

#[readonly::make]
#[derive(Default)]
pub struct SerialManager {
    #[readonly]
    pub available_port_names: Vec<SerialPortNames>,

    active_port: std::sync::Mutex<Option<Box<dyn serialport::SerialPort>>>,
    test_write_port: std::sync::Mutex<Option<Box<dyn serialport::SerialPort>>>,
    test_read_port: std::sync::Mutex<Option<Box<dyn serialport::SerialPort>>>,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SerialPortNames {
    name: String,
    manufacturer_name: Option<String>,
    product_name: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadioTestResult {
    packets_attempted: u32,
    packets_written: u32,
    packets_read: u32,
}

impl SerialManager {
    pub fn refresh_available_ports(&mut self) -> Result<bool, serialport::Error> {
        let new_available_ports = serialport::available_ports()?
            .into_iter()
            .filter_map(|port| match port.port_type {
                serialport::SerialPortType::UsbPort(usb_info) => Some(SerialPortNames {
                    name: port.port_name,
                    manufacturer_name: usb_info.manufacturer,
                    product_name: usb_info.product,
                }),
                serialport::SerialPortType::PciPort
                | serialport::SerialPortType::BluetoothPort
                | serialport::SerialPortType::Unknown => None,
            })
            .collect();

        if new_available_ports == self.available_port_names {
            return Ok(false);
        }

        self.available_port_names = new_available_ports;
        Ok(true)
    }

    pub fn set_active_port(&mut self, port_name: &str) -> Result<()> {
        Self::set_port(&mut self.active_port, port_name)
    }

    pub fn set_test_write_port(&mut self, port_name: &str) -> Result<()> {
        Self::set_port(&mut self.test_write_port, port_name)
    }

    pub fn set_test_read_port(&mut self, port_name: &str) -> Result<()> {
        Self::set_port(&mut self.test_read_port, port_name)
    }

    fn set_port(
        port_mutex: &mut std::sync::Mutex<Option<Box<dyn serialport::SerialPort>>>,
        port_name: &str,
    ) -> Result<()> {
        let port_mutex_result = port_mutex.lock();
        if port_mutex_result.is_err() {
            bail!("Failed to lock mutex!");
        }

        let mut port = port_mutex_result.unwrap();

        if port.is_some() {
            if let Some(active_port_name) = port.as_ref().unwrap().name() {
                if active_port_name == port_name {
                    // Do nothing if the given port is already active
                    return Ok(());
                }
            }
        }

        *port = Some(serialport::new(port_name, 9600).open()?);

        // Workaround issue where first packet sent to test port is not received due to a timeout
        port.as_deref_mut()
            .unwrap()
            .set_timeout(std::time::Duration::new(1, 0))?;

        Ok(())
    }

    pub fn write_test_packet_to_test_port(&self) -> Result<RadioTestResult> {
        let mut test_write_port_optional = match self.test_write_port.lock() {
            Ok(test_port) => test_port,
            Err(_) => bail!("Failed to lock mutex!"),
        };

        let packet_count = 1;

        if test_write_port_optional.is_none() {
            return Ok(RadioTestResult {
                packets_attempted: packet_count,
                packets_written: 0,
                packets_read: 0,
            });
        }

        let test_write_port = test_write_port_optional.as_mut().unwrap();

        test_write_port.write(&u32::to_le_bytes(42))?;
        test_write_port.write(&u32::to_le_bytes(0xFFFFFFFF))?;

        let packets_written = packet_count;

        let mut test_read_port_optional = match self.test_read_port.lock() {
            Ok(test_port) => test_port,
            Err(_) => bail!("Failed to lock mutex!"),
        };

        if test_read_port_optional.is_none() {
            return Ok(RadioTestResult {
                packets_attempted: packet_count,
                packets_written: packets_written,
                packets_read: 0,
            });
        }

        let test_read_port = test_read_port_optional.as_mut().unwrap();

        let mut buffer = [0; 5];

        let packets_read = match test_read_port.read_exact(&mut buffer) {
            Ok(_) => 1,
            Err(error) => {
                println!("{}", error.to_string());
                0
            }
        };

        Ok(RadioTestResult {
            packets_attempted: packet_count,
            packets_written,
            packets_read,
        })
    }

    pub fn read_from_active_port(&self, callback: &mut dyn FnMut(&[u8])) -> Result<()> {
        let mut active_port_optional = match self.active_port.lock() {
            Ok(active_port) => active_port,
            Err(_) => bail!("Failed to lock mutex!"),
        };

        if active_port_optional.is_none() {
            return Ok(());
        }

        let active_port = active_port_optional.as_mut().unwrap();

        let mut buffer = [0; 1024];

        while active_port.bytes_to_read().unwrap_or(0) > 0 {
            let bytes_read = active_port.read(&mut buffer)?;
            callback(&buffer[..bytes_read]);
        }

        Ok(())
    }
}
