use crate::use_packet_structure_manager;
use tauri::{AppHandle, Manager};

use crate::{
    packet_structure_manager::PacketStructureManager,
    packet_structure_manager_state::PacketStructureManagerState,
    packet_view_model::create_packet_view_model,
};

fn emit_packet_structure_update_event(
    app_handle: &tauri::AppHandle,
    packet_view_model_indices: Vec<usize>,
    packet_structure_manager: &PacketStructureManager,
) {
    let mut packet_view_models = Vec::with_capacity(packet_view_model_indices.len());
    for packet_view_model_index in packet_view_model_indices {
        packet_view_models.push(create_packet_view_model(
            &packet_structure_manager.packet_structures[packet_view_model_index],
        ));
    }

    app_handle
        .emit_all("packet-structures-update", &packet_view_models)
        .unwrap();
}

pub fn update_packet_structures(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    callback: &mut dyn FnMut(
        &mut PacketStructureManager,
    ) -> Result<Vec<usize>, (Vec<usize>, String)>,
) -> Result<(), String> {
    use_packet_structure_manager(
        &packet_structure_manager_state,
        &mut |packet_structure_manager| {
            let result = callback(packet_structure_manager);
            match result {
                Ok(modified_packet_view_model_indices) => {
                    emit_packet_structure_update_event(
                        &app_handle,
                        modified_packet_view_model_indices,
                        packet_structure_manager,
                    );
                    Ok(())
                }
                Err((modified_packet_view_model_indices, message)) => {
                    emit_packet_structure_update_event(
                        &app_handle,
                        modified_packet_view_model_indices,
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
                packet_structure_manager,
            );
            Ok(())
        },
    ) {
        Ok(_) => {}
        Err(_) => panic!("Failed to send initial packet stuctures!"),
    };
}
