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
pub fn start_radio_test(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    app_handle: tauri::AppHandle,
    send_port: &str,
    send_interval: u64
) -> Result<(), String> {
    println!("Starting radio test: send port: {} interval: {}", send_port, send_interval);

    use_serial_manager(serial_manager_state, &mut |serial_manager| {
        if !send_port.is_empty() {
            serial_manager.start_send_test(app_handle.clone(), send_port, Duration::from_millis(send_interval))?;
        }
        Ok(())
    })
}

#[tauri::command(async)]
pub fn stop_radio_test(
    serial_manager_state: tauri::State<'_, SerialManagerState>
) -> Result<(), String> {
    println!("Stopping radio test");

    use_serial_manager(serial_manager_state, &mut |serial_manager| {
        serial_manager.stop_send_test();
        Ok(())
    })
}
