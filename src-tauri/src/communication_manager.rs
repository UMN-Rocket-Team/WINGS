use serde::Serialize;

use crate::communication_drivers::{
    serial_port_driver::SerialPortDriver, 
    teledongle_driver::TeleDongleDriver,
};

#[derive(PartialEq, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SerialPortNames {
    pub name: String,
    pub manufacturer_name: Option<String>,
    pub product_name: Option<String>,
}

pub trait CommsIF {
    fn init_device(&mut self, port_name: &str)  -> anyhow::Result<()>;
    fn write_port(&mut self, packet: &[u8])  -> anyhow::Result<()>;
    fn read_port(&mut self, write_buffer: &mut Vec<u8>) -> anyhow::Result<()>;
    fn get_new_available_ports(&mut self) -> std::option::Option<Vec<SerialPortNames>>;
    fn has_port(&mut self) -> bool;
    fn set_id(&mut self, id: usize);
    fn get_id(&mut self) -> usize;
}

pub struct GetDataResult {
    pub(crate) new_ports: Option<Vec<SerialPortNames>>,
    pub(crate) data_read: Vec<u8>
}

pub struct CommunicationManager{
    pub comms_objects: Vec<Box<dyn CommsIF + Send>>,
    pub selected: usize,
}

impl Default for CommunicationManager{

    fn default() -> Self { 
        let receive: SerialPortDriver = Default::default();
        let send: SerialPortDriver = Default::default();
        Self{
            comms_objects: vec![Box::new(receive) as Box<dyn CommsIF + Send>, Box::new(send) as Box<dyn CommsIF + Send>],
            selected: 0,
        }
    }
}

impl CommunicationManager {

    /// Get data from the currently selected device
    /// 
    /// # Error
    /// 
    /// Returns an error if the device being addressed isn't initialized correctly
    pub fn get_data(&mut self, index: usize) -> Result<GetDataResult, String>{
        let mut result: GetDataResult = GetDataResult {
            new_ports: None,
            data_read: vec![]
        };

        let new_ports = self.comms_objects[index].get_new_available_ports();
        result.new_ports = new_ports;

        if self.comms_objects[index].has_port() {
            match self.comms_objects[index].read_port(&mut result.data_read) {
                Ok(_) => return Ok(result),
                Err(error) => return Err(error.to_string())
            }
        }

        Ok(result)
    }

    /// Write data to the currently selected device
    /// 
    /// # Errors
    /// 
    /// Returns an error if the device being addressed isn't initialized correctly
    pub fn write_data(&mut self, packet: &[u8], index: usize) -> anyhow::Result<()> {
        match self.comms_objects[index].write_port(packet){
            Ok(_) => Ok(()),
            Err(message) => Err(message)
        }
    }

    /// Connects the selected device struct to its hardware counterpart
    /// 
    /// # Errors
    /// 
    /// Was unable to initialize the device object
    pub fn init_device(&mut self, port_name: &str, index: usize) -> anyhow::Result<()>{
        match self.comms_objects[index].init_device(port_name){
            Ok(_) => Ok(()),
            Err(message) => Err(message)
        }
    }

    /// Adds an rfd device object to the manager
    pub fn add_rfd(&mut self){
        let mut new_device: SerialPortDriver = Default::default();
        let mut id = 0;
        for device in &mut self.comms_objects{
            if device.get_id() == id{
                id += 1;
            }
        }
        new_device.set_id(id);
        self.comms_objects.push(Box::new(new_device) as Box<dyn CommsIF + Send>);
    }

    /// Adds an altus metrum device object to the manager
    pub fn add_altus_metrum(&mut self){
        let mut new_device: TeleDongleDriver = Default::default();
        let mut id = 0;
        for device in &mut self.comms_objects{
            if device.get_id() == id{
                id += 1;
            }
        }
        new_device.set_id(id);
        self.comms_objects.push(Box::new(new_device) as Box<dyn CommsIF + Send>);
    }
}
