use std::sync::Mutex;

use crate::{mutex_utils::use_state_in_mutex, serial::SerialManager};

#[derive(Default)]
pub struct SerialManagerState {
    serial_manager: Mutex<SerialManager>,
}

pub fn use_serial_manager<ReturnType>(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    callback: &mut dyn FnMut(&mut SerialManager) -> Result<ReturnType, anyhow::Error>,
) -> Result<ReturnType, String> {
    use_state_in_mutex(&serial_manager_state.serial_manager, callback)
}
