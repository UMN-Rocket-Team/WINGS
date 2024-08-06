use crate::{models::packet_view_model::PacketStructureViewModel, state::generic_state::{result_to_error, use_struct, DataProcessorState, PacketStructureManagerState}};
use anyhow::{Error,anyhow};
use tauri::{AppHandle, Manager};
use serde::Serialize;

use crate::{
    packet_structure_manager::PacketStructureManager,
    models::packet_view_model::create_packet_view_model,
};

#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
pub enum PacketStructureViewModelUpdate {
    CreateOrUpdate(PacketStructureViewModel),
    Delete(usize)
}

fn emit_packet_structure_update_event(
    app_handle: &tauri::AppHandle,
    created_or_updated_packet_view_model_ids: Vec<usize>,
    deleted_packet_view_model_ids: Option<Vec<usize>>,
    packet_structure_manager: &PacketStructureManager,
) {
    let mut packet_view_model_updates = Vec::with_capacity(created_or_updated_packet_view_model_ids.len());
    for packet_view_model_id in created_or_updated_packet_view_model_ids {
        packet_view_model_updates.push(PacketStructureViewModelUpdate::CreateOrUpdate(create_packet_view_model(
            &packet_structure_manager.get_packet_structure(packet_view_model_id).unwrap(),
        )));
    }

    if let Some(deleted_packet_view_model_ids) = deleted_packet_view_model_ids {
        for packet_view_model_id in deleted_packet_view_model_ids {
            packet_view_model_updates.push(PacketStructureViewModelUpdate::Delete(packet_view_model_id));
        }
    }

    app_handle
        .emit_all("packet-structures-update", &packet_view_model_updates)
        .unwrap();
}

pub fn update_packet_structures(
    app_handle: tauri::AppHandle,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    data_processor_state: tauri::State<'_, DataProcessorState>,
    callback: &mut dyn FnMut(
        &mut PacketStructureManager,
    ) -> Result<(Vec<usize>, Option<Vec<usize>>), (Vec<usize>, Option<Vec<usize>>, String)>,
) -> Result<Result<(), Error>,String> {
    use_struct(
        &packet_structure_manager_state,
        &mut |packet_structure_manager| {
            let result = callback(packet_structure_manager);
            match result {
                Ok((modified_packet_view_model_ids, deleted_packet_view_model_ids)) => {
                    emit_packet_structure_update_event(
                        &app_handle,
                        modified_packet_view_model_ids,
                        deleted_packet_view_model_ids,
                        packet_structure_manager,
                    );
                }
                Err((modified_packet_view_model_ids, deleted_packet_view_model_ids, message)) => {
                    emit_packet_structure_update_event(
                        &app_handle,
                        modified_packet_view_model_ids,
                        deleted_packet_view_model_ids,
                        packet_structure_manager,
                    );
                    return Err(anyhow!(message));
                }
            }
            result_to_error(use_struct(
                &data_processor_state, 
                &mut |data_processor| {
                    match data_processor.generate_display_field_names(packet_structure_manager) {
                        Ok(new_fields) => {
                            app_handle
                                .emit_all("display-fields-update", new_fields)
                                .unwrap();
                            Ok(())
                        }
                        Err((new_fields,message)) => {
                            app_handle
                                .emit_all("display-fields-update", new_fields)
                                .unwrap();
                            Err(message)
                        }
                    }
                }
            ))
        },
    )
}

pub fn send_initial_packet_structure_update_event(app_handle: AppHandle) {
    match use_struct::<PacketStructureManager,()>(
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
        },
    ) {
        Ok(_) => {}
        Err(_) => panic!("Failed to send initial packet stuctures!"),
    };
}
