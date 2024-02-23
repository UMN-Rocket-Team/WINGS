use std::sync::Mutex;

use crate::{file_handling::FileHandler, state::mutex_utils::use_state_in_mutex};

#[derive(Default)]
pub struct FileHandlingState {
    csv_manager: Mutex<FileHandler>
}

pub fn use_file_handler<ReturnType>(
    file_handler_state: &tauri::State<'_, FileHandlingState>,
    callback: &mut dyn FnMut(&mut FileHandler) -> Result<ReturnType, anyhow::Error>,
) -> Result<ReturnType, String> {
    use_state_in_mutex(&file_handler_state.csv_manager, callback)
}
