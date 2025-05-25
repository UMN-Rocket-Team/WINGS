use crate::{
    data_processing::DataProcessorState, file_handling::config_struct::ConfigState, models::{packet_structure::{
    PacketFieldType, PacketMetadataType
    },
    packet_view_model::{PacketComponentType, PacketStructureViewModel},
    }
};
// # packet_structure_manager_commands
// 
// Contains all tauri commands related to the packet structure manager
// 
// !!!!!
// Everything in this file is Depreciated and should not be used moving forward. 
//
// Developers: If you want you could replace all of these calls with a json config file editor, but dont edit the PSM state directly
// !!!!!
//
// These functions update the current packet structures in the packet_Structure_manager_state, by calling update_packet_structures
// 

#[allow(unused_variables,unreachable_code)]
#[tauri::command]
pub fn set_packet_name(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    name: &str,
) -> Result<(), String> {
    return Err("WINGS can no longer edit packet structures, edit config instead".to_owned());
    todo!();
}

#[allow(unused_variables,unreachable_code)]
#[tauri::command]
pub fn set_field_name(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    field_index: usize,
    name: &str,
) -> Result<(), String> {
    return Err("WINGS can no longer edit packet structures, edit config instead".to_owned());
    todo!();
}

#[allow(unused_variables,unreachable_code)]
#[tauri::command]
pub fn set_field_type(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    field_index: usize,
    r#type: PacketFieldType,
) -> Result<(), String> {
    return Err("WINGS can no longer edit packet structures, edit config instead".to_owned());
    todo!();
}

#[allow(unused_variables,unreachable_code)]
#[tauri::command]
pub fn set_field_metadata_type(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    field_index: usize,
    metadata_type: PacketMetadataType,
) -> Result<(), String> {
    return Err("WINGS can no longer edit packet structures, edit config instead".to_owned());
    todo!();
}

#[allow(unused_variables,unreachable_code)]
#[tauri::command]
pub fn set_delimiter_name(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    delimiter_index: usize,
    name: &str,
) -> Result<(), String> {
    return Err("WINGS can no longer edit packet structures, edit config instead".to_owned());
    todo!();
}

#[allow(unused_variables,unreachable_code)]
#[tauri::command]
pub fn set_delimiter_identifier(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    delimiter_index: usize,
    identifier: &str,
) -> Result<(), String> {
    return Err("WINGS can no longer edit packet structures, edit config instead".to_owned());
    todo!();
}

#[allow(unused_variables,unreachable_code)]
#[tauri::command]
pub fn set_gap_size(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    gap_offset: usize,
    new_gap_size: isize,
) -> Result<(), String> {
    return Err("WINGS can no longer edit packet structures, edit config instead".to_owned());
    todo!();
}

#[allow(unused_variables,unreachable_code)]
#[tauri::command]
pub fn add_field(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
) -> Result<(), String> {
    return Err("WINGS can no longer edit packet structures, edit config instead".to_owned());
    todo!();
}

#[allow(unused_variables,unreachable_code)]
#[tauri::command]
pub fn add_delimiter(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
) -> Result<(), String> {
    return Err("WINGS can no longer edit packet structures, edit config instead".to_owned());
    todo!();
}

#[allow(unused_variables,unreachable_code)]
#[tauri::command]
pub fn add_gap_after(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    is_field: bool,
    component_index: usize,
) -> Result<(), String> {
    return Err("WINGS can no longer edit packet structures, edit config instead".to_owned());
    todo!();
}

#[allow(unused_variables,unreachable_code)]
#[tauri::command]
pub fn delete_packet_structure_component(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    component_index: usize,
    component_type: PacketComponentType,
) -> Result<(), String> {
    return Err("WINGS can no longer edit packet structures, edit config instead".to_owned());
    todo!();
}

/// Takes PacketStructureViewModel and parses it into a packetStructure, it then registers the packetStructure via the packet_structure_manager
///
/// ### Arguments
/// * 'view' - PacketStructureViewModel containing the packet that will be added to the packet structure
#[allow(unused_variables,unreachable_code)]
#[tauri::command]
pub fn add_packet_structure(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    view: PacketStructureViewModel,
) -> Result<(), String> {
    return Err("WINGS can no longer edit packet structures, edit config instead".to_owned());
    todo!();
}

#[allow(unused_variables,unreachable_code)]
#[tauri::command]
pub fn register_empty_packet_structure(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
) -> Result<(), String> {
    return Err("WINGS can no longer edit packet structures, edit config instead".to_owned());
    todo!();
}

#[allow(unused_variables,unreachable_code)]
#[tauri::command]
pub fn delete_packet_structure(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
) -> Result<(), String> {
    return Err("WINGS can no longer edit packet structures, edit config instead".to_owned());
    todo!();
}
