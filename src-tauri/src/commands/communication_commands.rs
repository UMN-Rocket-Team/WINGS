//! Tauri commands for managing communication devices.
//!
//! This module provides async commands for adding, initializing, and deleting
//! communication devices, as well as updating the frontend with device changes.
//! It interacts with the CommunicationManager and emits updates to the frontend.

use tauri::{AppHandle, Manager};

use crate::{
    communication_manager::{CommunicationManager, CommunicationManagerState},
    state::{generic_state::result_to_string, mutex_utils::use_state_in_mutex},
};
const COM_DEVICE_UPDATE: &str = "com-device-update";

/// Helper function for sending out an update of all coms manager devices.
/// Emits the current list of devices to the frontend.
fn update_coms(app_handle: &AppHandle, communication_manager: &mut CommunicationManager) {
    let mut return_me = vec![];
    communication_manager.update_display_com_devices(&mut return_me);
    let success = app_handle.emit_all(COM_DEVICE_UPDATE, &return_me);
    //notify devs if backend is failing to send updates to the frontend
    if success.is_err() {
        println!("WARNING: communication_commands.rs failed to communicate with frontend, \n| Warning Error:{:#?}",success)
    }
}

/// Deletes a device from the communication manager by its ID.
///
/// Emits an update to the frontend after deletion.
///
/// # Arguments
/// * `app_handle` - The Tauri app handle.
/// * `communication_manager_state` - The shared state of the communication manager.
/// * `id` - The ID of the device to delete.
///
/// # Returns
/// Result<(), String> - Ok on success, Err with error message on failure.
#[tauri::command(async)]
pub fn delete_device(
    app_handle: AppHandle,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
    id: usize,
) -> Result<(), String> {
    result_to_string(use_state_in_mutex(
        &communication_manager_state,
        &mut |communication_manager: &mut CommunicationManager| {
            let result = communication_manager.delete_device(id);
            update_coms(&app_handle, communication_manager);
            result
        },
    ))
}

/// Initializes a device port with the given name, baud rate, and ID.
///
/// # Arguments
/// * `communication_manager_state` - The shared state of the communication manager.
/// * `port_name` - The name of the port to initialize.
/// * `baud` - The baud rate for the device.
/// * `id` - The ID of the device to initialize.
///
/// # Returns
/// Result<(), String> - Ok on success, Err with error message on failure.
#[tauri::command(async)]
pub fn init_device_port(
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
    port_name: &str,
    baud: u32,
    id: usize,
) -> Result<(), String> {
    result_to_string(use_state_in_mutex(
        &communication_manager_state,
        &mut |communication_manager: &mut CommunicationManager| {
            println!("initializing! {} {} {}", port_name, baud, id);
            communication_manager.init_device(port_name, baud, id)
        },
    ))
}

/// Adds a new RFD (serial) device to the communication manager.
///
/// Emits an update to the frontend after addition.
///
/// # Arguments
/// * `app_handle` - The Tauri app handle.
/// * `communication_manager_state` - The shared state of the communication manager.
///
/// # Returns
/// Result<(), String> - Always Ok.
#[tauri::command(async)]
pub fn add_rfd(
    app_handle: AppHandle,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
) -> Result<(), String> {
    use_state_in_mutex(
        &communication_manager_state,
        &mut |communication_manager: &mut CommunicationManager| {
            communication_manager.add_serial_device();
            update_coms(&app_handle, communication_manager);
        },
    );
    Ok(())
}

/// Adds a new Altus Metrum device to the communication manager.
///
/// Emits an update to the frontend after addition.
///
/// # Arguments
/// * `app_handle` - The Tauri app handle.
/// * `communication_manager_state` - The shared state of the communication manager.
///
/// # Returns
/// Result<(), String> - Always Ok.
#[tauri::command(async)]
pub fn add_altus_metrum(
    app_handle: tauri::AppHandle,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
) -> Result<(), String> {
    use_state_in_mutex(
        &communication_manager_state,
        &mut |communication_manager: &mut CommunicationManager| {
            communication_manager.add_altus_metrum();
            update_coms(&app_handle, communication_manager);
        },
    );
    Ok(())
}

/// Adds a new file manager device to the communication manager and initializes it with the given file path.
///
/// Emits an update to the frontend after addition.
///
/// # Arguments
/// * `app_handle` - The Tauri app handle.
/// * `file_path` - The path to the file to use for the device.
/// * `communication_manager_state` - The shared state of the communication manager.
///
/// # Returns
/// Result<(), String> - Always Ok.
#[tauri::command(async)]
pub fn add_file_manager(
    app_handle: tauri::AppHandle,
    file_path: &str,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
) -> Result<(), String> {
    use_state_in_mutex(
        &communication_manager_state,
        &mut |communication_manager: &mut CommunicationManager| {
            let new_id = communication_manager.add_file_manager();
            let _ = communication_manager.init_device(file_path, 0, new_id);
            update_coms(&app_handle, communication_manager);
        },
    );
    Ok(())
}

/// Adds a new AIM device to the communication manager.
///
/// Emits an update to the frontend after addition.
///
/// # Arguments
/// * `app_handle` - The Tauri app handle.
/// * `communication_manager_state` - The shared state of the communication manager.
///
/// # Returns
/// Result<(), String> - Always Ok.
#[tauri::command(async)]
pub fn add_aim(
    app_handle: AppHandle,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
) -> Result<(), String> {
    use_state_in_mutex(
        &communication_manager_state,
        &mut |communication_manager: &mut CommunicationManager| {
            communication_manager.add_aim();
            update_coms(&app_handle, communication_manager);
        },
    );
    Ok(())
}

/// Adds a new Featherweight device to the communication manager.
///
/// Emits an update to the frontend after addition.
///
/// # Arguments
/// * `app_handle` - The Tauri app handle.
/// * `communication_manager_state` - The shared state of the communication manager.
///
/// # Returns
/// Result<(), String> - Always Ok.
#[tauri::command(async)]
pub fn add_featherweight(
    app_handle: AppHandle,
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
) -> Result<(), String> {
    use_state_in_mutex(
        &communication_manager_state,
        &mut |communication_manager: &mut CommunicationManager| {
            communication_manager.add_featherweight();
            update_coms(&app_handle, communication_manager);
        },
    );
    Ok(())
}
