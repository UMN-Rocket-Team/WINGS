use std::time::Duration;

use crate::{sending_loop::SendingModes, state::sending_loop_state::{use_sending_loop_manager, SendingLoopState}};

#[tauri::command(async)]
pub fn start_sending_loop(
    app_handle: tauri::AppHandle,
    test_manager_state: tauri::State<'_, SendingLoopState>,
    interval: u64,
    already_sent: u32,
    mode : SendingModes
) -> Result<(), String> {
    use_sending_loop_manager(test_manager_state, &mut |test_manager| {
        test_manager.start(app_handle.clone(), Duration::from_millis(interval), already_sent ,mode)
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