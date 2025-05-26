use crate::{file_handling::log_handlers::{FileHandlingState, LogHandler}, state::{_string, mutex_utils::use_state_in_mutex}};

///
/// Obsolete command used to set the file which the sending loop would read from
/// 
#[tauri::command(async)]
pub fn set_read(
    file_handler_state: tauri::State<'_, FileHandlingState>,
    path: &str,
) -> Result<(), String> {
    result_to_string(use_state_in_mutex(&file_handler_state, &mut |file_handler: &mut LogHandler| {
        file_handler.set_read(path.to_string())
    }))
}
