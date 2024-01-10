use std::time::Duration;

use crate::state::sending_loop_state::{SendingLoopState, use_sending_loop_manager};

#[tauri::command(async)]
pub fn start_sending_loop(
    app_handle: tauri::AppHandle,
    test_manager_state: tauri::State<'_, SendingLoopState>,
    interval: u64
) -> Result<(), String> {
    use_sending_loop_manager(test_manager_state, &mut |test_manager| {
        test_manager.start(app_handle.clone(), Duration::from_millis(interval))
    })
}

#[tauri::command(async)]
pub fn stop_sending_loop(
    test_manager_state: tauri::State<'_, SendingLoopState>
) -> Result<(), String> {
    use_sending_loop_manager(test_manager_state, &mut |test_manager| {
        test_manager.stop()
    })
}
