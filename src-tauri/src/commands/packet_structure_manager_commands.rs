use crate::{
    models::packet_structure::{PacketFieldType, PacketMetadataType},
    packet_structure_events::update_packet_structures,
    packet_structure_manager::{SetDelimiterIdentifierError, DeletePacketStructureComponentError},
    packet_structure_manager_state::PacketStructureManagerState, packet_view_model::{PacketComponentType, PacketViewModel},
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
            Ok(vec![packet_structure_id])
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
            Ok(vec![packet_structure_id])
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
            Ok(vec![packet_structure_id])
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
            Ok(vec![packet_structure_id])
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
            Ok(_) => Ok(vec![packet_structure_id]),
            Err(SetDelimiterIdentifierError::InvalidHexadecimalString(message)) => {
                Err((vec![packet_structure_id], message))
            }
            Err(SetDelimiterIdentifierError::IdentifierCollision(
                colliding_packet_structure_ids,
            )) => Err((
                colliding_packet_structure_ids,
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
            Ok(vec![packet_structure_id])
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
            Ok(vec![packet_structure_id])
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
            Ok(vec![packet_structure_id])
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
            Ok(vec![packet_structure_id])
        },
    )
}

#[tauri::command]
pub fn delete_packet_structure_component(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_structure_id: usize,
    component_index: usize,
    component_type: PacketComponentType
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            match packet_structure_manager.delete_packet_structure_component(packet_structure_id, component_index, component_type) {
                Ok(_) => {},
                Err(error) => return match error {
                    DeletePacketStructureComponentError::LastField => Err((vec![], String::from("Last Field"))),
                    DeletePacketStructureComponentError::LastDelimiter => Err((vec![], String::from("Last Delimiter"))),
                    DeletePacketStructureComponentError::DelimiterIdentifierCollision(identifiers) => Err((identifiers, String::from("Identifier collision"))),
                }
            }
            Ok(vec![packet_structure_id])
        }
    )
}

#[tauri::command]
pub fn add_packet(app_handle: tauri::AppHandle,// pointing to the location of the app in memory?
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,// the main datastructure of evertything in the app
    view: PacketViewModel // packet to be added(currently in json formatting)
) -> Result<(), String> {
    update_packet_structures(
        app_handle,
        packet_structure_manager_state,
        &mut |packet_structure_manager| {
            let new_id = packet_structure_manager.get_len();
            let packet_structure = view.to_packet_structure(new_id);
            match packet_structure_manager.register_packet_structure(packet_structure.clone()) {
                Ok(_) => {Ok(vec![new_id])}
                Err(_) => { 
                    Err((vec![new_id - 1],"Failed to register imported packet structures!".to_string()))
                }
            }
        }
    )
}


#[tauri::command]
pub fn debug(
    debug: &str
){
    print!("Running debug: ");
    println!("{debug}");
}