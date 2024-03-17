#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub type ULONG = ::std::os::raw::c_ulong;//20,700
pub type DWORD = ::std::os::raw::c_ulong;//20,710
pub type BOOL = ::std::os::raw::c_int;
pub type BYTE = ::std::os::raw::c_uchar;
pub type WORD = ::std::os::raw::c_ushort;
pub type FLOAT = f32;
pub type PFLOAT = *mut FLOAT;
pub type LPDWORD = *mut DWORD;//20,730
pub type LPVOID = *mut ::std::os::raw::c_void;
pub type LPCVOID = *const ::std::os::raw::c_void;
pub type UINT8 = ::std::os::raw::c_uchar;
pub type UINT16 = ::std::os::raw::c_ushort;//21,161
pub type UINT32 = ::std::os::raw::c_uint;
pub type UINT64 = ::std::os::raw::c_ulonglong;
pub type PVOID = *mut ::std::os::raw::c_void;//21,200
pub type FT_HANDLE = PVOID;//328,770
pub type FT_STATUS = ULONG;//328,770

extern "C" {//328,830
    pub fn FT_Open(deviceNumber: ::std::os::raw::c_int, pHandle: *mut FT_HANDLE) -> FT_STATUS;
}
extern "C" {
    pub fn FT_OpenEx(pArg1: PVOID, Flags: DWORD, pHandle: *mut FT_HANDLE) -> FT_STATUS;
}
extern "C" {
    pub fn FT_ListDevices(pArg1: PVOID, pArg2: PVOID, Flags: DWORD) -> FT_STATUS;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ft_device_list_info_node {//334.69
    pub Flags: ULONG,
    pub Type: ULONG,
    pub ID: ULONG,
    pub LocId: DWORD,
    pub SerialNumber: [u8; 16usize],
    pub Description: [u8; 64usize],
    pub ftHandle: FT_HANDLE,
}

pub type FT_DEVICE_LIST_INFO_NODE = _ft_device_list_info_node;//334.79

extern "C" {//334,798
    pub fn FT_CreateDeviceInfoList(lpdwNumDevs: LPDWORD) -> FT_STATUS;
}
extern "C" {
    pub fn FT_GetDeviceInfoDetail(
        dwIndex: DWORD,
        lpdwFlags: LPDWORD,
        lpdwType: LPDWORD,
        lpdwID: LPDWORD,
        lpdwLocId: LPDWORD,
        lpSerialNumber: LPVOID,
        lpDescription: LPVOID,
        pftHandle: *mut FT_HANDLE,
    ) -> FT_STATUS;
}
extern "C" {
    pub fn FT_GetDriverVersion(ftHandle: FT_HANDLE, lpdwVersion: LPDWORD) -> FT_STATUS;
}


pub type uint8 = UINT8;
pub type uint16 = UINT16;
pub type uint32 = UINT32;
pub type uint64 = UINT64;//334,885
pub type FT4222_STATUS = ULONG;//334,892
pub type FT4222_ClockRate = ::std::os::raw::c_int;
pub type FT4222_FUNCTION = ::std::os::raw::c_int;
pub type FT4222_SPIMode = ::std::os::raw::c_int;
pub type FT4222_SPIClock = ::std::os::raw::c_int;
pub type FT4222_SPICPOL = ::std::os::raw::c_int;
pub type FT4222_SPICPHA = ::std::os::raw::c_int;
pub type SPI_DrivingStrength = ::std::os::raw::c_int;
pub type SPI_ChipSelect = ::std::os::raw::c_int;
pub type GPIO_Dir = ::std::os::raw::c_int;
pub type GPIO_Port = ::std::os::raw::c_int;
pub type GPIO_Trigger = ::std::os::raw::c_int;
pub type GPIO_Output = ::std::os::raw::c_int;
pub type SPI_SlaveProtocol = ::std::os::raw::c_int;


extern "C" {
    pub fn FT4222_UnInitialize(ftHandle: FT_HANDLE) -> FT4222_STATUS;
}
extern "C" {
    pub fn FT4222_SetClock(ftHandle: FT_HANDLE, clk: FT4222_ClockRate) -> FT4222_STATUS;
}
extern "C" {
    pub fn FT4222_SetWakeUpInterrupt(ftHandle: FT_HANDLE, enable: BOOL) -> FT4222_STATUS;
}
extern "C" {
    pub fn FT4222_SetInterruptTrigger(ftHandle: FT_HANDLE, trigger: GPIO_Trigger) -> FT4222_STATUS;
}
extern "C" {//335,118
    pub fn FT4222_SetSuspendOut(ftHandle: FT_HANDLE, enable: BOOL) -> FT4222_STATUS;
}
extern "C" {
    pub fn FT4222_GetMaxTransferSize(ftHandle: FT_HANDLE, pMaxSize: *mut uint16) -> FT4222_STATUS;
}
extern "C" {
    pub fn FT4222_GPIO_Init(ftHandle: FT_HANDLE, gpioDir: *mut GPIO_Dir) -> FT4222_STATUS;
}
extern "C" {
    pub fn FT4222_GPIO_Write(
        ftHandle: FT_HANDLE,
        portNum: GPIO_Port,
        bValue: BOOL,
    ) -> FT4222_STATUS;
}
extern "C" {//335,200
    pub fn FT4222_SPISlave_InitEx(
        ftHandle: FT_HANDLE,
        protocolOpt: SPI_SlaveProtocol,
    ) -> FT4222_STATUS;
}
extern "C" {
    pub fn FT4222_SPISlave_SetMode(
        ftHandle: FT_HANDLE,
        cpol: FT4222_SPICPOL,
        cpha: FT4222_SPICPHA,
    ) -> FT4222_STATUS;
}
extern "C" {
    pub fn FT4222_SPISlave_GetRxStatus(ftHandle: FT_HANDLE, pRxSize: *mut uint16) -> FT4222_STATUS;
}
extern "C" {
    pub fn FT4222_SPISlave_Read(
        ftHandle: FT_HANDLE,
        buffer: *mut uint8,
        bufferSize: uint16,
        sizeOfRead: *mut uint16,
    ) -> FT4222_STATUS;
}
extern "C" {
    pub fn FT4222_SPISlave_Write(
        ftHandle: FT_HANDLE,
        buffer: *mut uint8,
        bufferSize: uint16,
        sizeTransferred: *mut uint16,
    ) -> FT4222_STATUS;
}
extern "C" {
    pub fn FT4222_SPI_Reset(ftHandle: FT_HANDLE) -> FT4222_STATUS;
}
extern "C" {
    pub fn FT4222_SPI_ResetTransaction(ftHandle: FT_HANDLE, spiIdx: uint8) -> FT4222_STATUS;
}
extern "C" {
    pub fn FT4222_SPI_SetDrivingStrength(
        ftHandle: FT_HANDLE,
        clkStrength: SPI_DrivingStrength,
        ioStrength: SPI_DrivingStrength,
        ssoStrength: SPI_DrivingStrength,
    ) -> FT4222_STATUS;
}
extern "C" {
    pub fn FT_Close(ftHandle: FT_HANDLE) -> FT_STATUS;
}