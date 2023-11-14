use std::sync::Mutex;

use crate::{mutex_utils::use_state_in_mutex, sending_loop::SendingLoop};

#[derive(Default)]
pub struct SendingLoopState {
    sending_loop_manager: Mutex<SendingLoop>
}

pub fn use_sending_loop_manager<ReturnType>(
    sending_loop_state: tauri::State<'_, SendingLoopState>,
    callback: &mut dyn FnMut(&mut SendingLoop) -> Result<ReturnType, anyhow::Error>,
) -> Result<ReturnType, String> {
    use_state_in_mutex(&sending_loop_state.sending_loop_manager, callback)
}
