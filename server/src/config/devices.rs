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
}


#[derive(Default, Copy, Clone)]
pub enum SerialDeviceRole {
    #[default]
    Transmitter,
    Reciever
}