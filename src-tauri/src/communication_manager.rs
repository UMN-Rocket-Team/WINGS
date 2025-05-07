use std::{collections::HashMap, sync::{Arc, Mutex}};

use anyhow::bail;
use hidapi::HidApi;
use serde::Serialize;

use crate::{
    communication_drivers::{
        aim_driver::AimDriver, byte_reader_driver::ByteReadDriver, serial_port_driver::SerialPortDriver, teledongle_driver::TeleDongleDriver
    }, file_handling::log_handlers::LogHandler, models::packet::Packet, packet_structure_manager::PacketStructureManager
};
#[derive(PartialEq, Serialize, Clone, Debug, Default, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct DeviceName {
    pub name: String,
    pub value: String,
    pub manufacturer_name: Option<String>,
    pub product_name: Option<String>,
}
#[derive(Serialize)]
pub struct DisplayComDevice {
    id: usize,
    device_type: String,
}

///A `Mutex` of `CommunicationManager`
pub type CommunicationManagerState = Mutex<CommunicationManager>;

#[derive(Default)]
pub struct CommunicationManager {
    pub comms_objects: Vec<Box<dyn CommsIF + Send>>,
    pub id_iterator: usize,
    pub old_device_names: Vec<DeviceName>,
    pub ps_manager: Arc<Mutex<PacketStructureManager>>,
    name_to_value: HashMap<String,String>,
}

pub trait CommsIF {
    fn new(
        packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
    ) -> Self 
    where
        Self: Sized;

    fn init_device(
        &mut self,
        port_name: &str,
        baud: u32,
    ) -> anyhow::Result<()>;
    fn write_port(&mut self, packet: &[u8]) -> anyhow::Result<()>;

    //Implements the communications side of the communications object (the bare minimum to get data), then returns it inside the Vec<u8>
    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()>;

    //Converts raw data into actual packets according to how the device specifies it
    fn parse_device_data(
        &mut self,
        raw_data_vector: &mut Vec<u8>,
        packet_vector: &mut Vec<Packet>,
    ) -> anyhow::Result<()>;
    fn get_device_packets(&mut self, data_vector: &mut Vec<Packet>) -> anyhow::Result<()>;
    fn is_init(&self) -> bool;
    fn set_id(&mut self, id: usize);
    fn get_id(&self) -> usize;
    fn get_type(&self) -> String;
}

impl CommunicationManager {
    pub fn default_state(ps_manager: Arc<Mutex<PacketStructureManager>>) -> CommunicationManagerState{
        let mut comms_manager = CommunicationManager::default();
        comms_manager.ps_manager = ps_manager.clone();
        Mutex::new(comms_manager)
    }

    ///checks for serial ports to connect to, streamlining radio setup
    pub fn get_all_potential_devices(&mut self) -> Option<Vec<DeviceName>> {
        let mut device_names: Vec<DeviceName>;
        let available_ports = serialport::available_ports().ok()?;
        let mut hid_api = HidApi::new().ok()?;
        _ = hid_api.refresh_devices();
        let available_hid_devices = hid_api.device_list();
        
        device_names = available_ports.into_iter()
            .filter_map(|port| match port.port_type {
                serialport::SerialPortType::UsbPort(usb_info) => {
                    let mut display_name = port.port_name.clone();
                    if usb_info.manufacturer.is_some(){
                        display_name = usb_info.manufacturer.clone().unwrap() + " " + &display_name;
                    } 
                    if usb_info.product.is_some(){
                        display_name = usb_info.product.clone().unwrap() + " " + &display_name;
                    }
                    Some(DeviceName {
                        name: display_name,
                        value: port.port_name,
                        manufacturer_name: usb_info.manufacturer,
                        product_name: usb_info.product,
                    })
                }
                _ => None,
            }).collect();
        
        let mut hid_devices: Vec<DeviceName> = available_hid_devices.filter_map(
            |device| 
            {
                let man_string =device.manufacturer_string().unwrap_or_default().to_owned();
                if man_string == "Microsoft" || man_string == "Logitech" || man_string == "Apple Inc." || man_string == "Apple" || man_string == "" {
                    return None;
                }
                return Some(DeviceName {
                    name: man_string+ " " + device.product_string().unwrap_or_default(),
                    value: device.path().to_str().unwrap_or_default().to_owned(),
                    manufacturer_name: Some(device.manufacturer_string().unwrap_or_default().to_owned()),
                    product_name: Some(device.product_string().unwrap_or_default().to_owned()), 
                });
            }
        ).collect();
        device_names.append(&mut hid_devices);
                
        if device_names == self.old_device_names {
            return None;
        } else {
            self.old_device_names = device_names.clone();
            for device in device_names.clone(){
                self.name_to_value.insert(device.name,device.value);
            }
            return Some(device_names);
        }
    }

    /// Checks if there is an initialized device with the given id
    /// Get data from the currently selected device
    ///
    /// # Error
    ///
    /// Returns an error if the device being addressed isn't initialized correctly
    pub fn get_data(&mut self, id: usize, return_buffer: &mut Vec<Packet>, log: &mut LogHandler) -> anyhow::Result<()> {
        let index = self.find(id,false).ok_or(anyhow::anyhow!("could not find a device with given id"))?;
        let mut raw_bytes = vec![];

        if !self.comms_objects[index].is_init() {
            return Err(anyhow::anyhow!("object not initialized"))
        }

        let result = self.comms_objects[index].get_device_raw_data(&mut raw_bytes);
        if result.is_err(){
            let error = result.unwrap_err();
            if !(format!("{}", error.root_cause()) == "Operation timed out"){
                return Err(error.context("failed to get raw data"));
            }
            else{
                return Ok(());
            }
        }
        let result = log.write_bytes(&raw_bytes,id,self.comms_objects[index].get_type());
        if result.is_err(){
            let new_result = result.unwrap_err().context("failed to write raw data");
            let context = new_result.chain();
            for i in context{
                eprintln!("Binary File Write{:#?}", i);
            }
        }

        let result = self.comms_objects[index].parse_device_data(&mut raw_bytes,return_buffer);
        if result.is_err(){
            return Err(result.unwrap_err().context("failed to get raw data"))
        }
        Ok(())
    }

    /// Write data to the currently selected device
    ///
    /// # Errors
    ///
    /// Returns an error if the device being addressed isn't initialized correctly
    pub fn write_data(&mut self, packet: &[u8], id: usize) -> anyhow::Result<()> {
        let index = self.find(id,false);
        match index {
            Some(index) => match self.comms_objects[index].write_port(packet) {
                Ok(_) => Ok(()),
                Err(message) => Err(message),
            },
            None => bail!(format!(
                "could not find a device with that ID: {} {}",
                id,
                self.comms_objects.len()
            )),
        }
    }

    /// Connects the selected device struct to its hardware counterpart
    ///
    /// # Errors
    ///
    /// Was unable to initialize the device object
    pub fn init_device(&mut self, port_name: &str, baud: u32, id: usize) -> anyhow::Result<()> {
        let index = self.find(id,false);
        match index {
            Some(index) => {
                let name;
                if self.comms_objects[index].get_type() == "ByteFile" || self.comms_objects[index].get_type() == "TeleDongle"{
                    name = port_name;
                }
                else{
                    name = self.name_to_value.get(port_name).ok_or(anyhow::anyhow!("Could not find a device with that name"))?;
                }
                match self.comms_objects[index].init_device(
                    name,
                    baud,
                ) {
                    Ok(_) => Ok(()),
                    Err(message) => Err(message)
                }
            },
            None => bail!(format!(
                "could not find a device with that ID: {} {}",
                id,
                self.comms_objects.len()
            )),
        }
    }

    /// Disconnects the selected device struct to its hardware counterpart
    ///
    /// # Errors
    ///
    /// Was unable to initialize the device object
    pub fn delete_device(&mut self, id: usize) -> anyhow::Result<()> {
        let index = self.find(id,true);
        match index {
            Some(index) => {
                self.comms_objects.remove(index);
                Ok(())
            }
            None => bail!(format!(
                "could not find a device with that ID: {} {}",
                id,
                self.comms_objects.len()
            )),
        }
    }

    /// Adds an rfd device object to the manager
    pub fn add_serial_device(&mut self) -> usize {
        let mut new_device: SerialPortDriver = SerialPortDriver::new(self.ps_manager.clone());
        new_device.set_id(self.id_iterator);
        self.id_iterator += 1;
        self.comms_objects
            .push(Box::new(new_device) as Box<dyn CommsIF + Send>);
        return self.comms_objects[self.comms_objects.len() - 1].get_id();
    }

    /// Adds an altus metrum device object to the manager
    pub fn add_altus_metrum(&mut self) -> usize {
        let mut new_device: TeleDongleDriver = TeleDongleDriver::new(self.ps_manager.clone());
        new_device.set_id(self.id_iterator);
        self.id_iterator += 1;
        self.comms_objects
            .push(Box::new(new_device) as Box<dyn CommsIF + Send>);
        return self.comms_objects[self.comms_objects.len() - 1].get_id();
    }

    /// Adds an byte reading device object to the manager
    pub fn add_file_manager(&mut self) -> usize {
        let mut new_device: ByteReadDriver = ByteReadDriver::new(self.ps_manager.clone());
        new_device.set_id(self.id_iterator);
        self.id_iterator += 1;
        self.comms_objects
            .push(Box::new(new_device) as Box<dyn CommsIF + Send>);
        return self.comms_objects[self.comms_objects.len() - 1].get_id();
    }

    /// Adds an byte reading device object to the manager
    pub fn add_aim(&mut self) -> usize {
        let mut new_device: AimDriver = AimDriver::new(self.ps_manager.clone());
        new_device.set_id(self.id_iterator);
        self.id_iterator += 1;
        self.comms_objects
            .push(Box::new(new_device) as Box<dyn CommsIF + Send>);
        return self.comms_objects[self.comms_objects.len() - 1].get_id();
    }

    //translates the device ID to array index
    fn find(&mut self, index: usize, print_flag: bool) -> Option<usize> {
        let mut i = 0;
        while i < self.comms_objects.len() {
            if print_flag {
                println!("{},{}",self.comms_objects[i].get_id(), index);
            }
            if self.comms_objects[i].get_id() == index {
                return Some(i);
            }
            i += 1;
        }
        None
    }


    /// The function `update_display_com_devices` iterates through communication objects, creates
    /// `DisplayComDevice` instances, and places them inside the provided buffer.
    pub fn update_display_com_devices(&mut self,buffer: &mut Vec<DisplayComDevice>){
        let mut i = 0;
        while i < self.comms_objects.len() {
            buffer.push(DisplayComDevice {
                id: self.comms_objects[i].get_id(),
                device_type: self.comms_objects[i].get_type(),
            });
            i += 1;
        }
    }

    //should be get_device_ids
    pub fn get_initialized_devices(&self) -> Vec<usize> {
        let mut return_me = vec![];
        for device in &self.comms_objects {
            if device.is_init(){
                return_me.push(device.get_id());
            }
        }
        return_me
    }
}


mod tests {


    #[test]
    #[ignore]
    fn test_if_serial_recognized() {
        use crate::communication_manager::CommunicationManager;

        let mut test_interface = CommunicationManager::default();
        let device_names = test_interface.get_all_potential_devices();
        assert!(device_names.is_some());
        assert!(device_names.unwrap().len() > 0);
    }

    #[test]
    #[ignore]
    fn test_serial_receive() {
        use crate::file_handling::log_handlers::LogHandler;
        use crate::{
            communication_manager::CommunicationManager,
            state::packet_structure_manager_state::default_packet_structure_manager,
        };
        use std::sync::Arc;
        
        const BAUD: u32 = 57600;

        let mut test_interface = CommunicationManager::default();
        let mut log_handler = LogHandler::default();
        log_handler.enable_debug();
        let device_names = test_interface.get_all_potential_devices();
        assert!(device_names.clone().is_some());
        assert!(device_names.clone().unwrap().len() > 0);
        test_interface.add_serial_device();
        test_interface.ps_manager = Arc::new(default_packet_structure_manager().into());
        let result = test_interface.init_device(&device_names.unwrap()[0].name, BAUD, 0);
        if result.is_err() {
            println!("{}", result.unwrap_err());
        };
        loop {
            let mut buff = vec![];
            let result = test_interface.get_data(0, &mut buff,&mut log_handler);
            println!("             {:#?}", result);
            println!("{:#?}", buff);
        }
    }
}
