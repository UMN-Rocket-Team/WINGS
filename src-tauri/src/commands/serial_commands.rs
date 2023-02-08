use crate::{
    serial::RadioTestResult,
    serial_manager_state::{use_serial_manager, SerialManagerState},
};

#[tauri::command]
pub fn set_active_port(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_serial_manager(serial_manager_state, &mut |usb_manager| {
        usb_manager.set_active_port(port_name)
    })
}

#[tauri::command]
pub fn set_test_write_port(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_serial_manager(serial_manager_state, &mut |usb_manager| {
        usb_manager.set_test_write_port(port_name)
    })
}

#[tauri::command]
pub fn set_test_read_port(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_serial_manager(serial_manager_state, &mut |usb_manager| {
        usb_manager.set_test_read_port(port_name)
    })
}

#[tauri::command]
pub fn test_radios(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
) -> Result<RadioTestResult, String> {
    use_serial_manager(serial_manager_state, &mut |usb_manager| {
        usb_manager.write_test_packet_to_test_port()
    })
}
