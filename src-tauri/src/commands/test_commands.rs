use std::time::Duration;

use crate::state::test_manager_state::{TestManagerState, use_test_manager};

#[tauri::command(async)]
pub fn start_radio_test(
    app_handle: tauri::AppHandle,
    test_manager_state: tauri::State<'_, TestManagerState>,
    interval: u64
) -> Result<(), String> {
    use_test_manager(test_manager_state, &mut |test_manager| {
        test_manager.start_radio_test(app_handle.clone(), Duration::from_millis(interval))
    })
}

#[tauri::command(async)]
pub fn stop_radio_test(
    test_manager_state: tauri::State<'_, TestManagerState>
) -> Result<(), String> {
    use_test_manager(test_manager_state, &mut |test_manager| {
        test_manager.stop_radio_test()
    })
}
