use crate::state::communication_state::{CommunicationManagerState, use_communication_manager};

// # serial_commands
//
// Contains all tauri commands related to the serial manager.
//
#[tauri::command(async)]
pub fn set_active_port(
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_communication_manager(communication_manager_state, &mut |communication_manager| {
        communication_manager.set_read_port(port_name)
    })
}

#[tauri::command(async)]
pub fn set_test_port(
    serial_manager_state: tauri::State<'_, CommunicationManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_communication_manager(serial_manager_state, &mut |communication_manager| {
        communication_manager.set_write_port(port_name)
    })
}
