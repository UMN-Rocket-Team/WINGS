use serde::Serialize;

use crate::communications::{
    serial_uart::SerialPortManager, 
    telemetrum::TeleManager,
};

#[derive(PartialEq, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SerialPortNames {
    pub name: String,
    pub manufacturer_name: Option<String>,
    pub product_name: Option<String>,
}

pub trait Communicatable {
    fn set_port(&mut self, port_name: &str)  -> anyhow::Result<()>;
    fn write_port(&mut self, packet: &[u8])  -> anyhow::Result<()>;
    fn read_port(&mut self, write_buffer: &mut Vec<u8>) -> anyhow::Result<()>;
    fn get_new_available_ports(&mut self) -> std::option::Option<Vec<SerialPortNames>>;
    fn has_port(&mut self) -> bool;
}

pub struct GetDataResult {
    pub(crate) new_ports: Option<Vec<SerialPortNames>>,
    pub(crate) data_read: Vec<u8>
}

pub struct CommunicationsManager{
    pub communicatables: Vec<Box<dyn Communicatable + Send>>,
    pub selected: usize,
}

impl Default for CommunicationsManager{

    fn default() -> Self { 
        let recieve: SerialPortManager = Default::default();
        let send: SerialPortManager = Default::default();
        Self{
            communicatables: vec![Box::new(recieve) as Box<dyn Communicatable + Send>, Box::new(send) as Box<dyn Communicatable + Send>],
            selected: 0,
        }
    }
}

impl CommunicationsManager {

    //get data from the currently selected device
    pub fn get_data(&mut self, index: usize) -> Result<GetDataResult, String>{
        let mut result: GetDataResult = GetDataResult {
            new_ports: None,
            data_read: vec![]
        };

        let new_ports = self.communicatables[index].get_new_available_ports();
        result.new_ports = new_ports;

        if self.communicatables[index].has_port() {
            match self.communicatables[index].read_port(&mut result.data_read) {
                Ok(_) => return Ok(result),
                Err(error) => return Err(error.to_string())
            }
        }

        Ok(result)
    }

    //write data to the currently selected device
    pub fn write_data(&mut self, packet: &[u8], index: usize) -> anyhow::Result<()> {
        match self.communicatables[index].write_port(packet){
            Ok(_) => Ok(()),
            Err(message) => Err(message)
        }
    }

    //set the port of the currently selected device object
    pub fn set_port(&mut self, port_name: &str, index: usize) -> anyhow::Result<()>{
        match self.communicatables[index].set_port(port_name){
            Ok(_) => Ok(()),
            Err(message) => Err(message)
        }
    }

    //adds an rfd device object to the manager
    pub fn add_rfd(&mut self){
        let new_device: SerialPortManager = Default::default();
        self.communicatables.push(Box::new(new_device) as Box<dyn Communicatable + Send>);
    }

    //adds an rfd device object to the manager
    pub fn add_altus_metrum(&mut self){
        let new_device: TeleManager = Default::default();
        self.communicatables.push(Box::new(new_device) as Box<dyn Communicatable + Send>);
    }
}
