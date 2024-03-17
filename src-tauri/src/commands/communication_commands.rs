use crate::state::communication_manager_state::{CommunicationManagerState, use_communication_manager};

// # serial_commands
//
// Contains all tauri commands related to the serial manager.
#[tauri::command(async)]
pub fn set_active_port(
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_communication_manager(communication_manager_state, &mut |communication_manager| {
        communication_manager.init_device(port_name, 0)
    })
}

#[tauri::command(async)]
pub fn set_test_port(
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
    port_name: &str,
) -> Result<(), String> {
    use_communication_manager(communication_manager_state, &mut |communication_manager| {
        communication_manager.init_device(port_name, 1)
    })
}

#[tauri::command(async)]
pub fn add_rfd(
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
) -> Result<(), String> {
    use_communication_manager(communication_manager_state, &mut |communication_manager| {
        communication_manager.add_rfd();
        Ok(())
    })
}

#[tauri::command(async)]
pub fn add_altus_metrum(
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
) -> Result<(), String> {
    use_communication_manager(communication_manager_state, &mut |communication_manager| {
        communication_manager.add_altus_metrum();
        Ok(())
    })
}
