use crate::communications::{
    serial_uart::SerialPortNames, 
    spi_ftdi::SerialFTDIManager, 
    telemetrum::TeleManager,
};


pub struct GetDataResult {
    pub(crate) new_ports: Option<Vec<SerialPortNames>>,
    pub(crate) data_read: Vec<u8>
}
#[derive(Default)]
#[allow(dead_code)]
pub struct CommunicationsManager {
    uart_manager : TeleManager,
    ftdi_manager : SerialFTDIManager,
    mode : i8
}
impl CommunicationsManager{
    pub fn get_data(&mut self) -> Result<GetDataResult, String>{
        let mut result: GetDataResult = GetDataResult {
            new_ports: None,
            data_read: vec![]
        };

        let new_ports = self.uart_manager.get_new_available_ports();
        result.new_ports = new_ports;

        if self.uart_manager.has_port() {
            match self.uart_manager.read_port(&mut result.data_read) {
                Ok(_) => return Ok(result),
                Err(error) => return Err(error.to_string())
            }
        }

        Ok(result)
    }
    pub fn write_data(&mut self, packet: &[u8]) -> anyhow::Result<()> {
        match self.uart_manager.write_port(packet){
            Ok(_) => Ok(()),
            Err(message) => Err(message)
        }
    }

    pub fn set_read_port(&mut self, port_name: &str) -> anyhow::Result<()>{
        match self.uart_manager.set_port(port_name){
            Ok(_) => Ok(()),
            Err(message) => Err(message)
        }
    }

    pub fn set_write_port(&mut self, port_name: &str) -> anyhow::Result<()>{
        match self.uart_manager.set_port(port_name){
            Ok(_) => Ok(()),
            Err(message) => Err(message)
        }
    }
}