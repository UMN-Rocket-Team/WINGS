use tauri::{AppHandle, Manager};

use crate::{communication_manager::{CommunicationManager,CommunicationManagerState}, state::generic_state::{result_to_string, use_struct}};
const COM_DEVICE_UPDATE: &str = "com-device-update";

///helper function for sending out an update of all coms manager devices
fn update_coms(
    app_handle: &AppHandle,
    communication_manager: &mut CommunicationManager){
    let mut return_me= vec![];
    communication_manager.update_display_com_devices(&mut return_me);
    let success = app_handle.emit_all(COM_DEVICE_UPDATE, &return_me);
    //notify devs if backend is failing to send updates to the frontend
    if success.is_err(){
        println!("WARNING: communication_commands.rs failed to communicate with frontend, \n| Warning Error:{:#?}",success)
    }
}

#[tauri::command(async)]
pub fn delete_device(
    app_handle: AppHandle,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
    id: usize,
) -> Result<(), String> {
    result_to_string(use_struct(&communication_manager_state, &mut |communication_manager: &mut CommunicationManager| {
        let result = communication_manager.delete_device(id);
        update_coms(&app_handle, communication_manager);
        return result;
         
    }))
}
// # serial_commands
//
// Contains all tauri commands related to the serial manager.
#[tauri::command(async)]
pub fn init_device_port(
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
    port_name: &str,
    baud: u32,
    id: usize,
) -> Result<(), String> {
    result_to_string(use_struct(&communication_manager_state, &mut |communication_manager: &mut CommunicationManager| {
        communication_manager.init_device(port_name, baud, id)
    }))
}

#[tauri::command(async)]
pub fn add_rfd(
    app_handle: AppHandle,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
) -> Result<(), String> {
    use_struct(&communication_manager_state, &mut |communication_manager: &mut CommunicationManager| {
        communication_manager.add_serial_device();
        update_coms(&app_handle, communication_manager);
    })
}

#[tauri::command(async)]
pub fn add_altus_metrum(
    app_handle: tauri::AppHandle,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
) -> Result<(), String> {
    use_struct(&communication_manager_state, &mut |communication_manager: &mut CommunicationManager| {
        communication_manager.add_altus_metrum();
        update_coms(&app_handle, communication_manager);
    })
}

#[tauri::command(async)]
pub fn add_file_manager(
    app_handle: tauri::AppHandle,
    file_path: &str,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
) -> Result<(), String> {
    use_struct(&communication_manager_state, &mut |communication_manager: &mut CommunicationManager| {
        let new_id =communication_manager.add_file_manager();
        let _ = communication_manager.init_device(file_path, 0, new_id);
        update_coms(&app_handle, communication_manager);
    })
}

#[tauri::command(async)]
pub fn add_aim(
    app_handle: AppHandle,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
) -> Result<(), String> {
    use_struct(&communication_manager_state, &mut |communication_manager: &mut CommunicationManager| {
        communication_manager.add_aim();
        update_coms(&app_handle, communication_manager);
    })
}

