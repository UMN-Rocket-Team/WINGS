use crate::{
    models::packet_structure::{
        PacketDelimiter, PacketField, PacketFieldType, PacketMetadataType, PacketStructure,
    },
    packet_structure_events::update_packet_structures,
    packet_structure_manager::{
        self, DeletePacketStructureComponentError, SetDelimiterIdentifierError,
    },
    packet_structure_manager_state::PacketStructureManagerState,
    packet_view_model::{PacketComponentType, PacketViewModel},
};

#[tauri::command]
pub fn set_field_name(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    field_index: usize,
    name: &str,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.set_field_name(packet_structure_id, field_index, name);
            Ok((vec![packet_structure_id], None))
        },
    )
}

#[tauri::command]
pub fn set_field_type(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    field_index: usize,
    r#type: PacketFieldType,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.set_field_type(packet_structure_id, field_index, r#type);
            Ok((vec![packet_structure_id], None))
        },
    )
}

#[tauri::command]
pub fn set_field_metadata_type(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    field_index: usize,
    metadata_type: PacketMetadataType,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.set_field_metadata_type(
                packet_structure_id,
                field_index,
                metadata_type,
            );
            Ok((vec![packet_structure_id], None))
        },
    )
}

#[tauri::command]
pub fn set_delimiter_name(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    delimiter_index: usize,
    name: &str,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.set_delimiter_name(packet_structure_id, delimiter_index, name);
            Ok((vec![packet_structure_id], None))
        },
    )
}

#[tauri::command]
pub fn set_delimiter_identifier(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    delimiter_index: usize,
    identifier: &str,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| match packet_structure_manager.set_delimiter_identifier(
            packet_structure_id,
            delimiter_index,
            identifier,
        ) {
            Ok(_) => Ok((vec![packet_structure_id], None)),
            Err(SetDelimiterIdentifierError::InvalidHexadecimalString(message)) => {
                Err((vec![packet_structure_id], None, message))
            }
            Err(SetDelimiterIdentifierError::IdentifierCollision(
                colliding_packet_structure_ids,
            )) => Err((
                colliding_packet_structure_ids,
                None,
                String::from("Identifiers must be unique between packet structures!"),
            )),
        },
    )
}

#[tauri::command]
pub fn set_gap_size(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    gap_index: usize,
    size: usize,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.set_gap_size(packet_structure_id, gap_index, size);
            Ok((vec![packet_structure_id], None))
        },
    )
}

#[tauri::command]
pub fn add_field(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.add_field(packet_structure_id);
            Ok((vec![packet_structure_id], None))
        },
    )
}

#[tauri::command]
pub fn add_delimiter(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.add_delimiter(packet_structure_id);
            Ok((vec![packet_structure_id], None))
        },
    )
}

#[tauri::command]
pub fn add_gap_after(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    is_field: bool,
    component_index: usize,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.add_gap_after(packet_structure_id, is_field, component_index);
            Ok((vec![packet_structure_id], None))
        },
    )
}

#[tauri::command]
pub fn delete_packet_structure_component(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    component_index: usize,
    component_type: PacketComponentType,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            match packet_structure_manager.delete_packet_structure_component(
                packet_structure_id,
                component_index,
                component_type,
            ) {
                Ok(_) => {}
                Err(error) => {
                    return match error {
                        DeletePacketStructureComponentError::LastField => {
                            Err((vec![], None, String::from("Last Field")))
                        }
                        DeletePacketStructureComponentError::LastDelimiter => {
                            Err((vec![], None, String::from("Last Delimiter")))
                        }
                        DeletePacketStructureComponentError::DelimiterIdentifierCollision(
                            identifiers,
                        ) => Err((identifiers, None, String::from("Identifier collision"))),
                    }
                }
            }
            Ok((vec![packet_structure_id], None))
        },
    )
}

/// Takes PackerViewModel and parses it into a packetStructure, it then registers the packetStructure via the packet_structure_manager
///
/// # Arguments
/// * 'view' - PackeViewModel containing the packet that will be added to the packet structure
#[tauri::command]
pub fn add_packet(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    view: PacketViewModel,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
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
    )
}

#[tauri::command]
pub fn register_empty_packet_structure(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
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
                    PacketField { index: 0, name: String::from("Field 1"), r#type: PacketFieldType::SignedInteger, offset_in_packet: 0, metadata_type: PacketMetadataType::None }
                ],
                delimiters: vec![
                    PacketDelimiter { index: 0, name: String::from("Delimiter 1"), identifier: smallest_delimiter, offset_in_packet: PacketFieldType::SignedInteger.size() }
                ],
            }) {
                Ok(new_id) => Ok((vec![new_id], None)),
                Err(error) => match error {
                    packet_structure_manager::PacketStructureRegistrationError::NameAlreadyRegistered(_) => Err((vec![], None, String::from("Unique name finding error: name collision!"))),
                    packet_structure_manager::PacketStructureRegistrationError::DelimitersAlreadyRegistered(_) => Err((vec![], None, String::from("Unique delimiter finding error: delimiter collision!"))),
                }
            }
        },
    )
}

#[tauri::command]
pub fn delete_packet_structure(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            packet_structure_manager.delete_packet_structure(packet_structure_id);
            Ok((vec![], Some(vec![packet_structure_id])))
        },
    )
}
