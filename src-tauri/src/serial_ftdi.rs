use libftd2xx::{Ftdi, FtdiCommon};
use tauri::Manager;

#[derive(Default)]
pub struct SerialFTDIManager {
    communciation_protocol: u8,
    isSyncronous: bool
}