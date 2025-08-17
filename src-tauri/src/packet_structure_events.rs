//! # Packet Structure Events
//!
//! This module handles the communication of packet structure changes from the Rust backend
//! to the Tauri frontend. It defines the data structures for the event payloads and provides
//! helper functions to construct and emit these events.
//!
//! Whenever a packet definition is created, updated, or deleted in the backend, the functions
//! in this module are called to notify all listening frontend windows of the change.

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

/// Represents a single update to the frontend's collection of packet structures.
///
/// This is a tagged enum designed for easy serialization and handling on the frontend.
/// It allows for sending different types of updates (creations, updates, deletions)
/// within a single event payload.
#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
pub enum PacketStructureViewModelUpdate {
    /// Informs the frontend to add a new packet structure or update an existing one.
    /// The payload is the complete, display-ready `PacketStructureViewModel`.
    CreateOrUpdate(PacketStructureViewModel),
    /// Informs the frontend to remove a packet structure.
    /// The payload is the unique ID of the packet structure to be deleted.
    Delete(usize),
}

/// Constructs and emits a "packet-structures-update" event to all frontend windows.
///
/// This function is the primary mechanism for synchronizing backend changes with the UI.
/// It takes lists of created/updated IDs and deleted IDs, converts them into a
/// vector of `PacketStructureViewModelUpdate` enums, and sends them in a single event.
///
/// # Arguments
/// * `app_handle` - A handle to the Tauri application, used to emit the event.
/// * `created_or_updated_packet_view_model_ids` - A vector of IDs for packets that have been newly created or modified.
///   The full `PacketStructureViewModel` for each of these will be generated and sent.
/// * `deleted_packet_view_model_ids` - An optional vector of IDs for packets that have been deleted.
/// * `packet_structure_manager` - A reference to the manager, used to retrieve the full packet data needed to create the view models.
pub fn emit_packet_structure_update_event(
    app_handle: &tauri::AppHandle,
    created_or_updated_packet_view_model_ids: Vec<usize>,
    deleted_packet_view_model_ids: Option<Vec<usize>>,
    packet_structure_manager: &PacketStructureManager,
) {
    let mut packet_view_model_updates =
        Vec::with_capacity(created_or_updated_packet_view_model_ids.len());

    // Process all creations and updates.
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

    // Emit the event with the collected updates to all frontend windows.
    app_handle
        .emit_all("packet-structures-update", &packet_view_model_updates)
        .unwrap();
}

/// Sends the complete list of all currently known packet structures to the frontend.
///
/// # Arguments
/// * `app_handle` - A handle to the Tauri application, used to access shared state and emit the event.
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
