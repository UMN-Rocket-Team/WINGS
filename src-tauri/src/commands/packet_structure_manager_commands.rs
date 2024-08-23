use crate::{
    generic_state::result_to_string, models::{packet_structure::{
        PacketDelimiter, PacketField, PacketFieldType, PacketMetadataType, PacketStructure,
    },
    packet_view_model::{PacketComponentType, PacketStructureViewModel},
    }, packet_structure_events::update_packet_structures, packet_structure_manager::Error, state::generic_state::{ConfigState, DataProcessorState}
};
// # packet_structure_manager_commands
// 
// Contains all tauri commands related to the packet structure manager
// 
// These functions update the current packet structures in the packet_Structure_manager_state, by calling update_packet_structures
// 
#[tauri::command]
pub fn set_packet_name(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    name: &str,
) -> Result<(), String> {
    result_to_string(update_packet_structures(
        app_handle,
        config_state,
        data_processor_state,
        &mut |packet_structure_manager| {
            match packet_structure_manager.set_packet_name(packet_structure_id, name) {
                Ok(()) => Ok((vec![packet_structure_id], None)),
                Err(err) => Err((vec![], None, err.to_string()))
            }
        },
    ))
}

#[tauri::command]
pub fn set_field_name(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    field_index: usize,
    name: &str,
) -> Result<(), String> {
    result_to_string(update_packet_structures(
        app_handle,
        config_state,
        data_processor_state,
        &mut |packet_structure_manager| {
            match packet_structure_manager.set_field_name(packet_structure_id, field_index, name) {
                Ok(()) => Ok((vec![packet_structure_id], None)),
                Err(err) => Err((vec![], None, err.to_string()))
            }
        },
    ))
}

#[tauri::command]
pub fn set_field_type(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    field_index: usize,
    r#type: PacketFieldType,
) -> Result<(), String> {
    result_to_string(update_packet_structures(
        app_handle,
        config_state,
        data_processor_state,
        &mut |packet_structure_manager| {
            match packet_structure_manager.set_field_type(packet_structure_id, field_index, r#type) {
                Ok(()) => Ok((vec![packet_structure_id], None)),
                Err(err) => Err((vec![], None, err.to_string()))
            }
        },
    ))
}

#[tauri::command]
pub fn set_field_metadata_type(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    field_index: usize,
    metadata_type: PacketMetadataType,
) -> Result<(), String> {
    result_to_string(update_packet_structures(
        app_handle,
        config_state,
        data_processor_state,
        &mut |packet_structure_manager| {
            match packet_structure_manager.set_field_metadata_type(
                packet_structure_id,
                field_index,
                metadata_type,
            ) {
                Ok(()) => Ok((vec![packet_structure_id], None)),
                Err(err) => Err((vec![], None, err.to_string()))
            }
        },
    ))
}

#[tauri::command]
pub fn set_delimiter_name(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    delimiter_index: usize,
    name: &str,
) -> Result<(), String> {
    result_to_string(update_packet_structures(
        app_handle,
        config_state,
        data_processor_state,
        &mut |packet_structure_manager| {
            match packet_structure_manager.set_delimiter_name(packet_structure_id, delimiter_index, name) {
                Ok(()) => Ok((vec![packet_structure_id], None)),
                Err(err) => Err((vec![], None, err.to_string()))
            }
        },
    ))
}

#[tauri::command]
pub fn set_delimiter_identifier(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    delimiter_index: usize,
    identifier: &str,
) -> Result<(), String> {
    result_to_string(update_packet_structures(
        app_handle,
        config_state,
        data_processor_state,
        &mut |packet_structure_manager| match packet_structure_manager.set_delimiter_identifier(
            packet_structure_id,
            delimiter_index,
            identifier,
        ) {
            Ok(_) => Ok((vec![packet_structure_id], None)),
            Err(error) => Err((vec![packet_structure_id], None, error.to_string()))
        },
    ))
}

#[tauri::command]
pub fn set_gap_size(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    gap_offset: usize,
    new_gap_size: isize,
) -> Result<(), String> {
    result_to_string(update_packet_structures(
        app_handle,
        config_state,
        data_processor_state,
        &mut |packet_structure_manager| {
            match packet_structure_manager.set_gap_size(packet_structure_id, gap_offset, new_gap_size) {
                Ok(()) => Ok((vec![packet_structure_id], None)),
                Err(err) => Err((vec![], None, err.to_string()))
            }
        },
    ))
}

#[tauri::command]
pub fn add_field(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
) -> Result<(), String> {
    result_to_string(update_packet_structures(
        app_handle,
        config_state,
        data_processor_state,
        &mut |packet_structure_manager| {
            match packet_structure_manager.add_field(packet_structure_id) {
                Ok(()) => Ok((vec![packet_structure_id], None)),
                Err(err) => Err((vec![], None, err.to_string()))
            }
        },
    ))
}

#[tauri::command]
pub fn add_delimiter(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
) -> Result<(), String> {
    result_to_string(update_packet_structures(
        app_handle,
        config_state,
        data_processor_state,
        &mut |packet_structure_manager| {
            match packet_structure_manager.add_delimiter(packet_structure_id) {
                Ok(()) => Ok((vec![packet_structure_id], None)),
                Err(err) => Err((vec![], None, err.to_string()))
            }
        },
    ))
}

#[tauri::command]
pub fn add_gap_after(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    is_field: bool,
    component_index: usize,
) -> Result<(), String> {
    result_to_string(update_packet_structures(
        app_handle,
        config_state,
        data_processor_state,
        &mut |packet_structure_manager| {
            match packet_structure_manager.add_gap_after(packet_structure_id, is_field, component_index) {
                Ok(()) => Ok((vec![packet_structure_id], None)),
                Err(err) => Err((vec![], None, err.to_string()))
            }
        },
    ))
}

#[tauri::command]
pub fn delete_packet_structure_component(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
    component_index: usize,
    component_type: PacketComponentType,
) -> Result<(), String> {
    result_to_string(update_packet_structures(
        app_handle,
        config_state,
        data_processor_state,
        &mut |packet_structure_manager| {
            match packet_structure_manager.delete_packet_structure_component(
                packet_structure_id,
                component_index,
                component_type,
            ) {
                Ok(_) => {}
                Err(error) => match error {
                    Error::DelimiterIdentifierCollision(ref ids) => {
                        return Err((ids.to_vec(), None, error.to_string()));
                    }
                    _ => {
                        return Err((vec![], None, error.to_string()));
                    }
                }
            }
            Ok((vec![packet_structure_id], None))
        },
    ))
}

/// Takes PacketStructureViewModel and parses it into a packetStructure, it then registers the packetStructure via the packet_structure_manager
///
/// ### Arguments
/// * 'view' - PacketStructureViewModel containing the packet that will be added to the packet structure
#[tauri::command]
pub fn add_packet_structure(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    view: PacketStructureViewModel,
) -> Result<(), String> {
    result_to_string(update_packet_structures(
        app_handle,
        config_state,
        data_processor_state,
        &mut |packet_structure_manager| {
            let mut packet_structure = view.to_packet_structure();
            match packet_structure_manager.register_packet_structure(&mut packet_structure) {
                Ok(new_id) => Ok((vec![new_id], None)),
                Err(_) => Err((
                    vec![],
                    None,
                    "Failed to register imported packet structures!".to_string(),
                )),
            }
        },
    ))
}

#[tauri::command]
pub fn register_empty_packet_structure(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
) -> Result<(), String> {
    result_to_string(update_packet_structures(
        app_handle,
        config_state,
        data_processor_state,
        &mut |packet_structure_manager| {
            // Find the smallest number possible to append to the name "New Packet " so that subsequent new packet's names do not collide
            let mut largest_new_packet_number: u32 = 0;

            for packet_structure in &packet_structure_manager.packet_structures {
                if packet_structure.name.starts_with("New Packet ") {
                    match packet_structure.name[11..].parse::<u32>() {
                        Ok(number) => {
                            if number > largest_new_packet_number {
                                largest_new_packet_number = number;
                            }
                        }
                        Err(_) => continue,
                    }
                }
            }

            largest_new_packet_number += 1;

            // Find a unique starting delimiter to identify to this packet structure
            let mut smallest_delimiter = vec![0];

            loop {
                for packet_structure in &packet_structure_manager.packet_structures {
                    if packet_structure.delimiters.len() == 1
                        && packet_structure.delimiters[0].identifier == smallest_delimiter
                    {
                        smallest_delimiter[0] += 1;
                        continue;
                    }
                }
                break;
            }

            match packet_structure_manager.register_packet_structure(&mut PacketStructure {
                id: 0,
                name: format!("New Packet {largest_new_packet_number}"),
                fields: vec![
                    PacketField { index: 0, name: String::from("Field 1"), r#type: PacketFieldType::SignedInteger, offset_in_packet: 0}
                ],
                delimiters: vec![
                    PacketDelimiter { index: 0, name: String::from("Delimiter 1"), identifier: smallest_delimiter, offset_in_packet: PacketFieldType::SignedInteger.size() }
                ],
                metafields: vec![],
            }) {
                Ok(new_id) => Ok((vec![new_id], None)),
                Err(error) => Err((vec![], None, error.to_string()))
            }
        },
    ))
}

#[tauri::command]
pub fn delete_packet_structure(
    app_handle: tauri::AppHandle,
    config_state: tauri::State<'_, ConfigState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    packet_structure_id: usize,
) -> Result<(), String> {
    result_to_string(update_packet_structures(
        app_handle,
        config_state,
        data_processor_state,
        &mut |packet_structure_manager| {
            match packet_structure_manager.delete_packet_structure(packet_structure_id) {
                Ok(()) => Ok((vec![], Some(vec![packet_structure_id]))),
                Err(err) => Err((vec![], None, err.to_string()))
            }
        },
    ))
}
