use crate::state::file_handling_state::{use_file_handler, FileHandlingState};

/// 
#[tauri::command(async)]
pub fn set_read(
    file_handler_state: tauri::State<'_, FileHandlingState>,
    path: &str,
) -> Result<(), String> {
    use_file_handler(&file_handler_state, &mut |file_handler| {
        file_handler.set_read(path.to_string())
    })
}
