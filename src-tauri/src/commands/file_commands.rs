use crate::state::generic_state::{result_to_string, use_struct, FileHandlingState};

/// 
#[tauri::command(async)]
pub fn set_read(
    file_handler_state: tauri::State<'_, FileHandlingState>,
    path: &str,
) -> Result<(), String> {
    result_to_string(use_struct(&file_handler_state, &mut |file_handler| {
        file_handler.set_read(path.to_string())
    }))
}
