use std::time::Duration;

use crate::{state::serial_manager_state::SerialManagerState, state::serial_manager_state::use_serial_manager};

// # serial_commands
//
// Contains all tauri commands related to the serial manager.
//
#[tauri::command(async)]
pub fn set_active_port(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_serial_manager(serial_manager_state, &mut |serial_manager| {
        serial_manager.set_active_port(port_name)
    })
}

#[tauri::command(async)]
pub fn set_test_port(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_serial_manager(serial_manager_state, &mut |serial_manager| {
        serial_manager.set_test_port(port_name)
    })
}
