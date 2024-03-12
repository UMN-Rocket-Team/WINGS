use std::sync::Mutex;

use crate::{communications_manager::CommunicationsManager, state::mutex_utils::use_state_in_mutex};
pub struct CommunicationManagerState {
    communication_manager: Mutex<CommunicationsManager>
}

pub fn use_communication_manager<ReturnType>(
    communication_manager_state: tauri::State<'_, CommunicationManagerState>,
    callback: &mut dyn FnMut(&mut CommunicationsManager) -> Result<ReturnType, anyhow::Error>,
) -> Result<ReturnType, String> 
{
    use_state_in_mutex(&communication_manager_state.communication_manager, callback)
}

impl Default for CommunicationManagerState {
    ///The default configuration for a packetStructureManager(the test packet you see when creating a new flight)
    fn default() -> Self {
        let coms_manager = Default::default();
        Self {
            communication_manager: Mutex::new(coms_manager),
        }
    }
}