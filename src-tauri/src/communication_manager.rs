
use anyhow::bail;
use serde::Serialize;
use tauri::Manager;

use crate::communication_drivers::{
    serial_port_driver::SerialPortDriver, 
    teledongle_driver::TeleDongleDriver,
};
#[derive(PartialEq, Serialize, Clone, Debug, Default, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct SerialPortNames {
    pub name: String,
    pub manufacturer_name: Option<String>,
    pub product_name: Option<String>,
}

const COM_DEVICE_UPDATE: &str = "com-device-update";
pub trait CommsIF {
    fn init_device(&mut self, port_name: &str)  -> anyhow::Result<()>;
    fn write_port(&mut self, packet: &[u8])  -> anyhow::Result<()>;
    fn read_port(&mut self, write_buffer: &mut Vec<u8>) -> anyhow::Result<()>;
    fn get_new_available_ports(&mut self) -> std::option::Option<Vec<SerialPortNames>>;
    fn has_port(&mut self) -> bool;
    fn set_id(&mut self, id: usize);
    fn get_id(&self) -> usize;
    fn get_type(&self) -> String;
}

pub struct GetDataResult {
    pub(crate) new_ports: Option<Vec<SerialPortNames>>,
    pub(crate) data_read: Vec<u8>
}
#[derive(Serialize)]
pub struct DisplayComDevice {
    id: usize,
    device_type: String,
}

pub struct CommunicationManager{
    pub comms_objects: Vec<Box<dyn CommsIF + Send>>,
    pub selected: usize,
    pub id_iterator: usize,
}

impl Default for CommunicationManager{

    fn default() -> Self { 
        Self{
            comms_objects: vec![],
            selected: 0,
            id_iterator: 0
        }
    }
}

impl CommunicationManager {

    /// Get data from the currently selected device
    /// 
    /// # Error
    /// 
    /// Returns an error if the device being addressed isn't initialized correctly
    pub fn get_data(&mut self, id: usize) -> Result<GetDataResult, String>{
        let index = self.find(id);
        match index{
            Some(index) => {
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
            },
            None => Err(format!("could not find a device with that ID: {} {}",id ,self.comms_objects.len())),
        }
    }

    /// Write data to the currently selected device
    /// 
    /// # Errors
    /// 
    /// Returns an error if the device being addressed isn't initialized correctly
    pub fn write_data(&mut self, packet: &[u8], id: usize) -> anyhow::Result<()> {
        let index = self.find(id);
        match index{
            Some(index) => match self.comms_objects[index].write_port(packet){
                Ok(_) => Ok(()),
                Err(message) => Err(message)
            },
            None => bail!(format!("could not find a device with that ID: {} {}",id ,self.comms_objects.len())),
        }
    }

    /// Connects the selected device struct to its hardware counterpart
    /// 
    /// # Errors
    /// 
    /// Was unable to initialize the device object
    pub fn init_device(&mut self, port_name: &str, id: usize) -> anyhow::Result<()>{
        let index = self.find(id);
        match index{
            Some(index) => 
                match self.comms_objects[index].init_device(port_name){
                    Ok(_) => Ok(()),
                    Err(message) => Err(message)
                },
            None => bail!(format!("could not find a device with that ID: {} {}",id ,self.comms_objects.len())),
        }
    }

    /// DisConnects the selected device struct to its hardware counterpart
    /// 
    /// # Errors
    /// 
    /// Was unable to initialize the device object
    pub fn delete_device(&mut self, id: usize) -> anyhow::Result<()>{
        let index = self.find(id);
        match index{
            Some(index) => 
                {self.comms_objects.remove(index);
                Ok(())},
            None => bail!(format!("could not find a device with that ID: {} {}",id ,self.comms_objects.len())),
        }
    }


    /// Adds an rfd device object to the manager
    pub fn add_rfd(&mut self){
        let mut new_device: SerialPortDriver = Default::default();
        new_device.set_id(self.id_iterator);
        self.id_iterator+=1;
        self.comms_objects.push(Box::new(new_device) as Box<dyn CommsIF + Send>);
    }

    /// Adds an altus metrum device object to the manager
    pub fn add_altus_metrum(&mut self){
        let mut new_device: TeleDongleDriver = Default::default();
        new_device.set_id(self.id_iterator);
        self.id_iterator+=1;
        self.comms_objects.push(Box::new(new_device) as Box<dyn CommsIF + Send>);

    }

    //translates the device ID to array index
    fn find(&mut self, index: usize) -> Option<usize>{
        let mut i = 0;
        while i < self.comms_objects.len(){
            if self.comms_objects[i].get_id() == index{
                return Some(i);
            }
            i+=1;
        }
        None
    }

    pub fn update_display_com_devices(&mut self, app_handle: tauri::AppHandle){
        let mut return_me = vec![];
        let mut i = 0;
        while i < self.comms_objects.len(){
            return_me.push(DisplayComDevice{ id: self.comms_objects[i].get_id(), device_type: self.comms_objects[i].get_type() });
            i+=1;
        }
        let _ = app_handle.emit_all(COM_DEVICE_UPDATE, &return_me);
    }
    
    pub fn get_devices(&self) -> Vec<usize>{
        let mut return_me = vec![];
        for device in &self.comms_objects{
            return_me.push(device.get_id());
        }
        return_me
    }
}
