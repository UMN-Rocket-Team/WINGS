use crate::state::communication_manager_state::{CommunicationManagerState, use_communication_manager};

#[tauri::command(async)]
pub fn delete_device(
    app_handle: tauri::AppHandle,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
    id: usize,
) -> Result<(), String> {
    use_communication_manager(communication_manager_state, &mut |communication_manager| {
        match communication_manager.delete_device(id){
            Ok(_) => {
                communication_manager.update_display_com_devices(app_handle.clone());
                Ok(())
            },
            Err(err) => {
                communication_manager.update_display_com_devices(app_handle.clone());
                Err(err)
            },
        }
    })
}
// # serial_commands
//
// Contains all tauri commands related to the serial manager.
#[tauri::command(async)]
pub fn init_device_port(
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
    port_name: &str,
    id: usize,
) -> Result<(), String> {
    use_communication_manager(communication_manager_state, &mut |communication_manager| {
        communication_manager.init_device(port_name, id)
    })
}

#[tauri::command(async)]
pub fn add_rfd(
    app_handle: tauri::AppHandle,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
) -> Result<(), String> {
    use_communication_manager(communication_manager_state, &mut |communication_manager| {
        communication_manager.add_rfd();
        communication_manager.update_display_com_devices(app_handle.clone());
        Ok(())
    })
}

#[tauri::command(async)]
pub fn add_altus_metrum(
    app_handle: tauri::AppHandle,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
) -> Result<(), String> {
    use_communication_manager(communication_manager_state, &mut |communication_manager| {
        communication_manager.add_altus_metrum();
        communication_manager.update_display_com_devices(app_handle.clone());
        Ok(())
    })
}

#[tauri::command(async)]
pub fn add_file_manager(
    app_handle: tauri::AppHandle,
    file_path: &str,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
) -> Result<(), String> {
    use_communication_manager(communication_manager_state, &mut |communication_manager| {
        let new_id =communication_manager.add_file_manager();
        let _ = communication_manager.init_device(file_path, new_id);
        communication_manager.update_display_com_devices(app_handle.clone());
        Ok(())
    })
}

