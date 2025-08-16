use crate::{
    file_handling::config_struct::ConfigState, models::packet_view_model::PacketStructureViewModel,
    state::mutex_utils::use_state_in_mutex,
};
use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::{
    models::packet_view_model::create_packet_view_model,
    packet_structure_manager::PacketStructureManager,
};

#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
pub enum PacketStructureViewModelUpdate {
    CreateOrUpdate(PacketStructureViewModel),
    Delete(usize),
}

pub fn emit_packet_structure_update_event(
    app_handle: &tauri::AppHandle,
    created_or_updated_packet_view_model_ids: Vec<usize>,
    deleted_packet_view_model_ids: Option<Vec<usize>>,
    packet_structure_manager: &PacketStructureManager,
) {
    let mut packet_view_model_updates =
        Vec::with_capacity(created_or_updated_packet_view_model_ids.len());
    for packet_view_model_id in created_or_updated_packet_view_model_ids {
        packet_view_model_updates.push(PacketStructureViewModelUpdate::CreateOrUpdate(
            create_packet_view_model(
                packet_structure_manager
                    .get_packet_structure(packet_view_model_id)
                    .unwrap(),
            ),
        ));
    }

    if let Some(deleted_packet_view_model_ids) = deleted_packet_view_model_ids {
        for packet_view_model_id in deleted_packet_view_model_ids {
            packet_view_model_updates
                .push(PacketStructureViewModelUpdate::Delete(packet_view_model_id));
        }
    }
    app_handle
        .emit_all("packet-structures-update", &packet_view_model_updates)
        .unwrap();
}

pub fn send_initial_packet_structure_update_event(app_handle: AppHandle) {
    use_state_in_mutex(&app_handle.state::<ConfigState>(), &mut |config| {
        emit_packet_structure_update_event(
            &app_handle,
            config
                .packet_structure_manager
                .packet_structures
                .iter()
                .map(|packet_structure| packet_structure.id)
                .collect(),
            None,
            &config.packet_structure_manager,
        );
    });
}
