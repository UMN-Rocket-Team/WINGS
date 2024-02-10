use std::sync::Mutex;

use crate::{csv_manager::CSVManager, state::mutex_utils::use_state_in_mutex};

#[derive(Default)]
pub struct CSVManagerState {
    csv_manager: Mutex<CSVManager>
}

pub fn use_csv_manager<ReturnType>(
    csv_manager_state: &tauri::State<'_, CSVManagerState>,
    callback: &mut dyn FnMut(&mut CSVManager) -> Result<ReturnType, anyhow::Error>,
) -> Result<ReturnType, String> {
    use_state_in_mutex(&csv_manager_state.csv_manager, callback)
}
