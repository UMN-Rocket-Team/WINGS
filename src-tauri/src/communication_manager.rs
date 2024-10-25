

use anyhow::{bail,anyhow};
use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::{communication_drivers::{
    byte_reader_driver::ByteReadDriver, serial_port_driver::SerialPortDriver, teledongle_driver::TeleDongleDriver
}, models::packet::Packet, state::generic_state::ConfigState};
#[derive(PartialEq, Serialize, Clone, Debug, Default, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct DeviceName {
    pub name: String,
    pub manufacturer_name: Option<String>,
    pub product_name: Option<String>,
}

const COM_DEVICE_UPDATE: &str = "com-device-update";
const HANDLE_EXPECT: &str = "This is assigned at initialization"; // reason to expect the app_handle in the code
pub trait CommsIF {
    fn init_device(&mut self, port_name: &str, baud: u32, app_handle: AppHandle)  -> anyhow::Result<()>;
    fn write_port(&mut self, packet: &[u8])  -> anyhow::Result<()>;

    //Implements the communications side of the communications object (the bare minimum to get data), then returns it inside the Vec<u8>
    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()>;

    //Converts raw data into actual packets according to how the device specifies it
    fn parse_device_data(&mut self, raw_data_vector: &mut Vec<u8>, packet_vector: &mut Vec<Packet>) -> anyhow::Result<()>;
    fn get_device_packets(&mut self, data_vector: &mut Vec<Packet>) -> anyhow::Result<()>;

    //for future plug and play implementations, could be deleted if necessary 
    fn get_new_available_ports(&mut self) -> std::option::Option<Vec<DeviceName>>;
    fn is_init(&mut self) -> bool;
    fn set_id(&mut self, id: usize);
    fn get_id(&self) -> usize;
    fn get_type(&self) -> String;
}
#[derive(Serialize)]
pub struct DisplayComDevice {
    id: usize,
    device_type: String,
}

#[derive(Default)]
pub struct CommunicationManager{
    pub comms_objects: Vec<Box<dyn CommsIF + Send>>,
    pub id_iterator: usize,
    pub old_device_names: Vec<DeviceName>,
    pub app_handle: Option<AppHandle>,//can always be expected since it will be assigned during initialization
}

impl CommunicationManager {

    /// Initializes all currently connected devices automatically
    /// 
    /// Initialize the Communication manager with any serial devices that are already attached to the ground station
    pub fn init(&mut self, app_handle: AppHandle){
        self.app_handle = Some(app_handle.clone());
        let maybe_device_names = self.get_all_potential_devices();
        match maybe_device_names{
            Some(devices) => {
                for device_name in devices{
                    let number = self.add_serial_device();
                    match self.init_device(&device_name.name , app_handle.state::<ConfigState>().lock().expect(HANDLE_EXPECT).default_baud, number) {
                        Ok(_) => {},
                        Err(_) => {_= self.delete_device(number)},
                    }
                }
            },
            None => {},
        }
        self.update_display_com_devices();
    }

    //for plug and play
    pub fn get_all_potential_devices(&mut self)-> Option<Vec<DeviceName>>{

        let available_ports ;
        match serialport::available_ports(){
            Ok(ports) => available_ports = ports,
            Err(_) => return None,
        }
        let device_names: Vec<DeviceName> = available_ports
            .into_iter()
            .filter_map(|port| match port.port_type {
                serialport::SerialPortType::UsbPort(usb_info) => {
                    // On macOS, each serial port shows up as both eg.:
                    //  - /dev/cu.usbserial-AK06O4AO
                    //  - /dev/tty.usbserial-AK06O4AO
                    // For our use, these are equivalent, so we'll filter one out to avoid confusion.
                    if cfg!(target_os = "macos") && port.port_name.starts_with("/dev/cu.usbserial-") {
                        None
                    } else {
                        Some(DeviceName {
                            name: port.port_name,
                            manufacturer_name: usb_info.manufacturer,
                            product_name: usb_info.product,
                        })
                    }
                },
                serialport::SerialPortType::PciPort
                | serialport::SerialPortType::BluetoothPort
                | serialport::SerialPortType::Unknown => None,
            })
            .collect();
        if device_names == self.old_device_names{
            return None;
        }
        else{
            self.old_device_names = device_names.clone();
            return Some(device_names)
        }
    }
    /// Get data from the currently selected device
    /// 
    /// # Error
    /// 
    /// Returns an error if the device being addressed isn't initialized correctly
    pub fn get_data(&mut self, id: usize, return_buffer: &mut Vec<Packet>) -> anyhow::Result<()>{
        let index = self.find(id);
        match index{
            Some(index) => {
                let mut result = vec![];

                if self.comms_objects[index].is_init() {
                    match self.comms_objects[index].get_device_packets(&mut result) {
                        Ok(_) => return_buffer.append(&mut result),
                        Err(error) => return Err(error)
                    }
                }

                Ok(())
            },
            None => bail!(format!("could not find a device with that ID: {} {}",id ,self.comms_objects.len())),
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
                Ok(_) => {Ok(())},
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
    pub fn init_device(&mut self, port_name: &str,baud: u32, id: usize) -> anyhow::Result<()>{
        let index = self.find(id);
        match index{
            Some(index) => 
                match self.comms_objects[index].init_device(port_name, baud,self.app_handle.clone().expect(HANDLE_EXPECT)){
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
    pub fn add_serial_device(&mut self)->usize{
        let mut new_device: SerialPortDriver = Default::default();
        new_device.set_id(self.id_iterator);
        self.id_iterator+=1;
        self.comms_objects.push(Box::new(new_device) as Box<dyn CommsIF + Send>);
        return self.comms_objects[self.comms_objects.len() - 1].get_id();
    }

    /// Adds an altus metrum device object to the manager
    pub fn add_altus_metrum(&mut self)->usize{
        let mut new_device: TeleDongleDriver = Default::default();
        new_device.set_id(self.id_iterator);
        self.id_iterator+=1;
        self.comms_objects.push(Box::new(new_device) as Box<dyn CommsIF + Send>);
        return self.comms_objects[self.comms_objects.len() - 1].get_id();

    }

    /// Adds an byte reading device object to the manager
    pub fn add_file_manager(&mut self)->usize{
        let mut new_device: ByteReadDriver = Default::default();
        new_device.set_id(self.id_iterator);
        self.id_iterator+=1;
        self.comms_objects.push(Box::new(new_device) as Box<dyn CommsIF + Send>);
        return self.comms_objects[self.comms_objects.len() - 1].get_id();

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

    pub fn update_display_com_devices(&mut self){
        let mut return_me = vec![];
        let mut i = 0;
        while i < self.comms_objects.len(){
            return_me.push(DisplayComDevice{ id: self.comms_objects[i].get_id(), device_type: self.comms_objects[i].get_type() });
            i+=1;
        }
        let _ = self.app_handle.clone().unwrap().emit_all(COM_DEVICE_UPDATE, &return_me);
    }

    //should be get_device_ids
    pub fn get_devices(&self) -> Vec<usize>{
        let mut return_me = vec![];
        for device in &self.comms_objects{
            return_me.push(device.get_id());
        }
        return_me
    }
}

mod tests {
    
    use crate::state::generic_state::{CommunicationManagerState, FileHandlingState};

    use super::*; // lets the unit tests use everything in this file
    use tauri::Manager;

    #[test]
    #[ignore]
    fn test_if_serial_recognized() {
        let mut test_interface = CommunicationManager::default();
        let device_names = test_interface.get_all_potential_devices();
        assert!(device_names.is_some());
        assert!(device_names.unwrap().len() > 0);
    }
    const BAUD: u32 = 57600;

    #[test]
    #[ignore]
    fn test_serial_receive() {
        let mut test_interface = CommunicationManager::default();
        let device_names = test_interface.get_all_potential_devices();
        assert!(device_names.clone().is_some());
        assert!(device_names.clone().unwrap().len() > 0);
        test_interface.add_serial_device();
        let result = test_interface.init_device(&device_names.unwrap()[0].name, BAUD, 0);
        if result.is_err(){
            println!("{}", result.unwrap_err());
        };
        loop{
            let mut buff = vec![];
            let result = test_interface.get_data(0,&mut buff);
            println!("             {:#?}",result);
            println!("{:#?}",buff);
        }
    }
}