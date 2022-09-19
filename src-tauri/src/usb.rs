#[readonly::make]
#[derive(Default)]
pub struct UsbManager {

    #[readonly]
    pub available_ports: Vec<SerialPort>,

    active_port: Option<std::sync::Mutex<Box<dyn serialport::SerialPort>>>,
    
}

#[derive(PartialEq)]
#[derive(serde::Serialize)]
pub struct SerialPort {

    name: String,
    manufacturer_name: Option<String>,
    product_name: Option<String>,

}

impl UsbManager {

    pub fn new() -> Self { Self { available_ports: vec![], active_port: None } }

    pub fn refresh_available_ports(&mut self) -> bool {
        let new_available_ports = serialport::available_ports().unwrap().into_iter().filter_map(|port| {
            match port.port_type {
                serialport::SerialPortType::UsbPort(usb_info) => {
                    Some(SerialPort {
                        name: port.port_name,
                        manufacturer_name: usb_info.manufacturer,
                        product_name: usb_info.product
                    })
                },
                serialport::SerialPortType::PciPort | serialport::SerialPortType::BluetoothPort | serialport::SerialPortType::Unknown => None,
            }
        }).collect();

        if new_available_ports == self.available_ports {
            return false;
        }

        self.available_ports = new_available_ports;
        return true;
    }

    pub fn set_active_port(&mut self, port_name: &str) -> Result<(), serialport::Error> {
        match serialport::new(port_name, 9600).open() {
            Ok(serial_port) => {
                self.active_port = Some(std::sync::Mutex::new(serial_port));
                return Ok(());
            },
            Err(error) => {
                self.active_port = None;
                return Err(error);
            },
        }
    }

    pub fn read_from_active_port(&self, callback: fn(&[u8])) {
        if let Some(active_port_mutex) = &self.active_port {
            let active_port = &mut *active_port_mutex.lock().unwrap();

            let mut buffer = [0; 1024];

            while active_port.bytes_to_read().unwrap_or(0) > 0 {
                match active_port.read(&mut buffer) {
                    Ok(bytes_read) => callback(&buffer[..bytes_read]),
                    Err(_) => todo!(),
                }
            }
        }
    }

}