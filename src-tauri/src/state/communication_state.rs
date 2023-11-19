use std::sync::Mutex;

use crate::{mutex_utils::use_state_in_mutex, communications_manager::CommunicationsManager};

#[derive(Default)]
pub struct CommunicationManagerState {
    communication_manager: Mutex<CommunicationsManager>
}

pub fn use_communication_manager<ReturnType>(
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
    callback: &mut dyn FnMut(&mut CommunicationsManager) -> Result<ReturnType, anyhow::Error>,
) -> Result<ReturnType, String> {
    use_state_in_mutex(&communication_manager_state.communication_manager, callback)
}
