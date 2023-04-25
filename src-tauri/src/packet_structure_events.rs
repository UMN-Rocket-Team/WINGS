use crate::{use_packet_structure_manager, packet_view_model::PacketViewModel};
use tauri::{AppHandle, Manager};
use serde::Serialize;

use crate::{
    packet_structure_manager::PacketStructureManager,
    packet_structure_manager_state::PacketStructureManagerState,
    packet_view_model::create_packet_view_model,
};

#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
pub enum PacketStructureViewModelUpdate {
    CreateOrUpdate(PacketViewModel),
    Delete(usize)
}

fn emit_packet_structure_update_event(
    app_handle: &tauri::AppHandle,
    created_or_updated_packet_view_model_indices: Vec<usize>,
    deleted_packet_view_model_indices: Option<Vec<usize>>,
    packet_structure_manager: &PacketStructureManager,
) {
    let mut packet_view_model_updates = Vec::with_capacity(created_or_updated_packet_view_model_indices.len());
    for packet_view_model_index in created_or_updated_packet_view_model_indices {
        packet_view_model_updates.push(PacketStructureViewModelUpdate::CreateOrUpdate(create_packet_view_model(
            &packet_structure_manager.packet_structures[packet_view_model_index],
        )));
    }

    if let Some(deleted_packet_view_model_indices) = deleted_packet_view_model_indices {
        for packet_view_model_index in deleted_packet_view_model_indices {
            packet_view_model_updates.push(PacketStructureViewModelUpdate::Delete(packet_view_model_index));
        }
    }

    app_handle
        .emit_all("packet-structures-update", &packet_view_model_updates)
        .unwrap();
}

pub fn update_packet_structures(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    callback: &mut dyn FnMut(
        &mut PacketStructureManager,
    ) -> Result<(Vec<usize>, Option<Vec<usize>>), (Vec<usize>, Option<Vec<usize>>, String)>,
) -> Result<(), String> {
    use_packet_structure_manager(
        &packet_structure_manager_state,
        &mut |packet_structure_manager| {
            let result = callback(packet_structure_manager);
            match result {
                Ok((modified_packet_view_model_indices, deleted_packet_view_model_indices)) => {
                    emit_packet_structure_update_event(
                        &app_handle,
                        modified_packet_view_model_indices,
                        deleted_packet_view_model_indices,
                        packet_structure_manager,
                    );
                    Ok(())
                }
                Err((modified_packet_view_model_indices, deleted_packet_view_model_indices, message)) => {
                    emit_packet_structure_update_event(
                        &app_handle,
                        modified_packet_view_model_indices,
                        deleted_packet_view_model_indices,
                        packet_structure_manager,
                    );
                    Err(message)
                }
            }
        },
    )
}

pub fn send_initial_packet_structure_update_event(app_handle: AppHandle) {
    match use_packet_structure_manager::<(), &str>(
        &app_handle.state::<PacketStructureManagerState>(),
        &mut |packet_structure_manager| {
            emit_packet_structure_update_event(
                &app_handle,
                packet_structure_manager
                    .packet_structures
                    .iter()
                    .map(|packet_structure| packet_structure.id)
                    .collect(),
                None,
                packet_structure_manager,
            );
            Ok(())
        },
    ) {
        Ok(_) => {}
        Err(_) => panic!("Failed to send initial packet stuctures!"),
    };
}
