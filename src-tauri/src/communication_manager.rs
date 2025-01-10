use std::sync::{Arc, Mutex};

use anyhow::bail;
use serde::Serialize;

use crate::{
    communication_drivers::{
        byte_reader_driver::ByteReadDriver, serial_port_driver::SerialPortDriver,
        teledongle_driver::TeleDongleDriver,
    },
    models::packet::Packet,
    packet_structure_manager::PacketStructureManager,
};
#[derive(PartialEq, Serialize, Clone, Debug, Default, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct DeviceName {
    pub name: String,
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
    pub ps_manager: Arc<PacketStructureManager>,
}

pub trait CommsIF {
    fn init_device(
        &mut self,
        port_name: &str,
        baud: u32,
        packet_structure_manager: Arc<PacketStructureManager>,
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
    fn is_init(&mut self) -> bool;
    fn set_id(&mut self, id: usize);
    fn get_id(&self) -> usize;
    fn get_type(&self) -> String;
}

impl CommunicationManager {
    pub fn default_state(ps_manager: PacketStructureManager) -> CommunicationManagerState{
        let mut comms_manager = CommunicationManager::default();
        comms_manager.ps_manager = Arc::new(ps_manager);
        Mutex::new(comms_manager)
    }

    ///checks for serial ports to connect to, streamlining radio setup
    pub fn get_all_potential_devices(&mut self) -> Option<Vec<DeviceName>> {
        let available_ports;
        match serialport::available_ports() {
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
                    if cfg!(target_os = "macos") && port.port_name.starts_with("/dev/cu.usbserial-")
                    {
                        None
                    } else {
                        Some(DeviceName {
                            name: port.port_name,
                            manufacturer_name: usb_info.manufacturer,
                            product_name: usb_info.product,
                        })
                    }
                }
                serialport::SerialPortType::PciPort
                | serialport::SerialPortType::BluetoothPort
                | serialport::SerialPortType::Unknown => None,
            })
            .collect();
        if device_names == self.old_device_names {
            return None;
        } else {
            self.old_device_names = device_names.clone();
            return Some(device_names);
        }
    }

    /// Get data from the currently selected device
    ///
    /// # Error
    ///
    /// Returns an error if the device being addressed isn't initialized correctly
    pub fn get_data(&mut self, id: usize, return_buffer: &mut Vec<Packet>) -> anyhow::Result<()> {
        let index = self.find(id,false);
        match index {
            Some(index) => {
                let mut result = vec![];

                if self.comms_objects[index].is_init() {
                    match self.comms_objects[index].get_device_packets(&mut result) {
                        Ok(_) => return_buffer.append(&mut result),
                        Err(error) => return Err(error),
                    }
                }

                Ok(())
            }
            None => bail!(format!(
                "could not find a device with that ID: {} {}",
                id,
                self.comms_objects.len()
            )),
        }
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
            Some(index) => match self.comms_objects[index].init_device(
                port_name,
                baud,
                self.ps_manager.clone(),
            ) {
                Ok(_) => Ok(()),
                Err(message) => Err(message)
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
        let mut new_device: SerialPortDriver = Default::default();
        new_device.set_id(self.id_iterator);
        self.id_iterator += 1;
        self.comms_objects
            .push(Box::new(new_device) as Box<dyn CommsIF + Send>);
        return self.comms_objects[self.comms_objects.len() - 1].get_id();
    }

    /// Adds an altus metrum device object to the manager
    pub fn add_altus_metrum(&mut self) -> usize {
        let mut new_device: TeleDongleDriver = Default::default();
        new_device.set_id(self.id_iterator);
        self.id_iterator += 1;
        self.comms_objects
            .push(Box::new(new_device) as Box<dyn CommsIF + Send>);
        return self.comms_objects[self.comms_objects.len() - 1].get_id();
    }

    /// Adds an byte reading device object to the manager
    pub fn add_file_manager(&mut self) -> usize {
        let mut new_device: ByteReadDriver = Default::default();
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
    pub fn get_devices(&self) -> Vec<usize> {
        let mut return_me = vec![];
        for device in &self.comms_objects {
            return_me.push(device.get_id());
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

        use crate::{
            communication_manager::CommunicationManager,
            state::packet_structure_manager_state::default_packet_structure_manager,
        };
        use std::sync::Arc;
        
        const BAUD: u32 = 57600;

        let mut test_interface = CommunicationManager::default();
        let device_names = test_interface.get_all_potential_devices();
        assert!(device_names.clone().is_some());
        assert!(device_names.clone().unwrap().len() > 0);
        test_interface.add_serial_device();
        test_interface.ps_manager = Arc::new(default_packet_structure_manager());
        let result = test_interface.init_device(&device_names.unwrap()[0].name, BAUD, 0);
        if result.is_err() {
            println!("{}", result.unwrap_err());
        };
        loop {
            let mut buff = vec![];
            let result = test_interface.get_data(0, &mut buff);
            println!("             {:#?}", result);
            println!("{:#?}", buff);
        }
    }
}
