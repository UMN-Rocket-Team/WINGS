#![allow(dead_code)]
use std::ptr::addr_of_mut;
use crate::ftdi4222::{FT_CreateDeviceInfoList, FT_DEVICE_LIST_INFO_NODE, FT_GetDeviceInfoDetail, LPVOID, FT_HANDLE, FT_OpenEx, DWORD, PVOID, BOOL, FT4222_SetSuspendOut, FT4222_GPIO_Init, FT4222_GPIO_Write, GPIO_Port, FT4222_UnInitialize, FT_Close, FT4222_SetClock, FT4222_SPISlave_InitEx, FT4222_SPI_SetDrivingStrength, FT4222_SPISlave_GetRxStatus, FT4222_SPISlave_Read, FT4222_SPISlave_SetMode, FT4222_SPISlave_Write, FT4222_GetMaxTransferSize};


//internal function to simplify error handling
fn ft_result<T>(value: T, status: u32) -> Result<T, u32> {
    if status != 0 {
        Err(status)
    } else {
        Ok(value)
    }
}
pub struct FTDeviceListInfoNode {
    pub node: FT_DEVICE_LIST_INFO_NODE
}
impl FTDeviceListInfoNode{

    /// Helper function to make it easier to access the Description of a chip
    /// 
    /// This is needed because the description is given as a c string and ends with a null terminator
    /// Rust does not use null termiantors and needs to be explicitly told that that is where the string ends.
    pub fn get_description(&self) ->  String{
        let description_raw = self.node.Description;
        let description = String::from_utf8_lossy(&description_raw).into_owned();
        return description[..(description.find('\0')).unwrap()].to_owned();
    }
}

pub struct FTHandle {
    handle: FT_HANDLE
}
impl FTHandle{
    
    /// enables the use of GPIO2 & the Led for debugging
    /// 
    /// suspendstate:
    ///     false - gpio2 can be used as gpio
    ///     true  - gpio2 will be used as a usb suspend output indicator(light will be on only when the chip is idle)
    pub fn ft4222_set_suspend_out(&self, suspend_state: bool) -> Result<u32,u32>{
        let result_var = unsafe { FT4222_SetSuspendOut(addr_of_mut!(*(self.handle)), suspend_state as BOOL)};
        ft_result(result_var, result_var)
    }

    /// Sets the clock rate of the chip
    /// 
    /// clock = 3; ->  80Mhz clock   & 20Mhz or less spi supported
    /// 
    /// clock = 0; ->  60Mhz clock   & 15Mhz or less spi supported  
    /// 
    /// clock = 2; ->  48Mhz clock   & 12Mhz or less spi supported  
    /// 
    /// clock = 1; ->  48Mhz clock   & 6Mhz or less spi supported  
    /// 
    pub fn ft4222_set_clock(&self,clock: i32) -> Result<u32,u32>{
        let result_var = unsafe { FT4222_SetClock(addr_of_mut!(*self.handle),clock)};
        ft_result(result_var, result_var)
    }

    pub fn ft4222_get_max_transfer_size(&self) -> Result<u16,u32>{
        let mut transfer_size: u16 = 0;
        let result_var = unsafe { FT4222_GetMaxTransferSize(addr_of_mut!(*self.handle),addr_of_mut!(transfer_size))};
        ft_result(transfer_size, result_var)
    }

    /// Initializes the chip in gpio mode
    /// 
    /// dirs - each element sets its corresponding GPIO port(0-3)
    ///        0 = output
    ///        1 = input 
    pub fn ft4222_gpio_init(&self, dirs: &mut [i32; 4]) -> Result<u32,u32>{
        let result_var = unsafe { FT4222_GPIO_Init(addr_of_mut!(*(self.handle)), addr_of_mut!(dirs[0]))};
        ft_result(result_var, result_var)
    }

    /// Changes the status of a gpio port to either on or off
    /// 
    /// port_num - 0 to 3
    /// bool_value - Either true or false to represent an on or off state
    pub fn ft4222_gpio_write(&self,port_num: i32, bool_value: bool) -> Result<u32,u32>{
        let result_var = unsafe { FT4222_GPIO_Write(addr_of_mut!(*(self.handle)), port_num as GPIO_Port, bool_value as BOOL)};
        ft_result(result_var, result_var)
    }

    /// sets the chip to SPI slave communication mode  
    /// 
    /// spi_slave_protocol - should pretty much always be 1 when used by wings  
    /// 
    /// spi_slave_protocol = 0   with protocol (this is a built in packet format, never use this)  
    /// 
    /// spi_slave_protocol = 1   no protocol (user is responsible for decripting packets, this is what we want)  
    /// 
    /// spi_slave_protocol = 2   no ack (similar to mode 0, dont use this)  
    pub fn ft4222_spi_slave_init_ex(&self,spi_slave_protocol: i32) -> Result<u32,u32>{
        let result_var = unsafe { FT4222_SPISlave_InitEx(addr_of_mut!(*self.handle),spi_slave_protocol)};
        ft_result(result_var, result_var)
    }

    /// sets the electric signal strength that the chip will be working at
    /// 
    /// clk_strength - strength at which the clock (sck for spi) will operate at
    /// io_strength  - strength of any incoming or outgoing signals
    /// sso_strength - strength of ant chip select signals
    /// 
    /// all inputs should be ints between 0 and 3
    /// 0 - 4 milliamps
    /// 1 - 8 milliamps
    /// 2 - 12 milliamps
    /// 3 - 16 milliamps
    pub fn ft4222_spi_set_driving_strength(&self,clk_strength: i32, io_strength: i32,sso_strength: i32) -> Result<u32,u32>{
        let result_var = unsafe { FT4222_SPI_SetDrivingStrength(addr_of_mut!(*self.handle),clk_strength,io_strength,sso_strength)};
        ft_result(result_var, result_var)
    }
    
    /// returns the size of the read buffer(bytes going ftdi to computer)
    pub fn ft4222_spi_slave_get_rx_status(&self) -> Result<u16,u32>{
        let mut rx_size:u16 = 0;
        let result_var = unsafe { FT4222_SPISlave_GetRxStatus(addr_of_mut!(*self.handle),addr_of_mut!(rx_size))};
        ft_result(rx_size, result_var)
    }

    /// transfers incomming bytes out of chip and into the given array
    /// 
    /// reads from the chips read buffer(bytes going ftdi to computer)
    /// places the bytes into the buffer given by input paramater
    /// 
    /// returns the amount of bytes transferred
    pub fn ft4222_spi_slave_read(&self,buffer: &mut [u8]) -> Result<u16,u32>{
        let mut size_of_read:u16 = 0;
        let result_var = unsafe { FT4222_SPISlave_Read(addr_of_mut!(*self.handle),addr_of_mut!(buffer[0]),buffer.len() as u16,addr_of_mut!(size_of_read))};
        ft_result(size_of_read, result_var)
    }

    /// copies outgoing bytes out of array and sends them to the chip
    /// 
    /// sends bytes from the given buffer onto the chip
    /// the bytes are then queued on the chip to be sent with the next clock cycle
    /// 
    /// returns the amount of bytes transferred
    pub fn ft4222_spi_slave_write(&self,buffer: &mut [u8]) -> Result<u16,u32>{
        let mut size_of_transfer:u16 = 0;
        let result_var = unsafe { FT4222_SPISlave_Write(addr_of_mut!(*self.handle),addr_of_mut!(buffer[0]),buffer.len() as u16,addr_of_mut!(size_of_transfer))};
        ft_result(size_of_transfer, result_var)
    }

    /// Sets the SPI mode at which the chip opperates
    /// 
    /// clock polarity - when true, idle high
    /// 
    /// clock phase - when true, bits are align to when the clock stops idling
    pub fn ft4222_spi_set_mode(&self,clock_polarity: bool, clock_phase: bool) -> Result<u32,u32>{
        let result_var = unsafe { FT4222_SPISlave_SetMode(addr_of_mut!(*(self.handle)), clock_polarity as i32, clock_phase as i32)};
        ft_result(result_var, result_var)
    }

    /// Closes the connection to the 4222 chip
    /// 
    /// Should be called at the end of every program in order to exit correctly
    pub fn ft4222_unitialize(&self) -> Result<u32,u32>{
        let result_var = unsafe { FT4222_UnInitialize(addr_of_mut!(*self.handle))};
        match result_var{
            0 => {
                let result_var = unsafe { FT_Close(addr_of_mut!(*self.handle))};
                ft_result(result_var, result_var)
            },
            _ => Err(result_var)
        }
    }
}


/// creates an internal list of ftdi devices that can be accessed with ft_get_info_detail()
/// 
/// returns the number of ftdi devices connected to the laptop
/// 
/// Official Docummentation:
/// This function builds a device information list and returns the number of D2XX devices connected to the
/// system. The list contains information about both unopen and open devices.
pub fn ft_create_device_info_list() -> Result<u32,u32>{
    let mut num_devs: DWORD = 0;
    let result_var = unsafe { FT_CreateDeviceInfoList(addr_of_mut!(num_devs)) };
    ft_result(num_devs, result_var)
}

/// returns an FTDeviceListInfoNode from the list created by ft_create_device_info_list()
/// 
/// index- index within the list, should be less than the output of ft_create_device_info_list()
pub fn ft_get_device_info_detail(index: u32,) -> Result<FTDeviceListInfoNode,u32>{
    let new_node: FT_DEVICE_LIST_INFO_NODE = FT_DEVICE_LIST_INFO_NODE{
        Flags: 0,
        Type: 0,
        ID: 0,
        LocId: 0,
        SerialNumber: [0; 16],
        Description: [0; 64],
        ftHandle: std::ptr::null_mut(),
    };
    let mut node_wrap = FTDeviceListInfoNode{
        node: new_node
    };

    let result_var = unsafe { FT_GetDeviceInfoDetail(index,addr_of_mut!(node_wrap.node.Flags), addr_of_mut!(node_wrap.node.Type), addr_of_mut!(node_wrap.node.ID), addr_of_mut!(node_wrap.node.LocId), 
        addr_of_mut!(node_wrap.node.SerialNumber) as LPVOID, 
        addr_of_mut!(node_wrap.node.Description) as LPVOID,
        addr_of_mut!(node_wrap.node.ftHandle) as *mut FT_HANDLE) 
    };

    ft_result(node_wrap, result_var)
}

/// returns a ft handle generated from the given info node
/// 
/// info_node- from ft_get_device_info_detail()
pub fn ft_open_ex(info_node: &FTDeviceListInfoNode) -> Result<FTHandle,u32>{
    let new_handle: FT_HANDLE = std::ptr::null_mut();
    let mut handle_wrap = FTHandle{
        handle: new_handle
    };
    let result_var = unsafe { FT_OpenEx(info_node.node.LocId as PVOID, 4 as DWORD, addr_of_mut!(handle_wrap.handle))};
    ft_result(handle_wrap, result_var)
}