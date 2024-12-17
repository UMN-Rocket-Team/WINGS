use std::time::Duration;

use crate::{sending_loop::{SendingLoopState, SendingModes}, state::generic_state::{result_to_string, use_struct}};

#[tauri::command(async)]
pub fn start_sending_loop(
    app_handle: tauri::AppHandle,
    sending_loop_state: tauri::State<'_, SendingLoopState>,
    interval: u64,
    already_sent: u32,
    mode : SendingModes,
    write_id: usize
) -> Result<(), String> {
    result_to_string(use_struct(&sending_loop_state, &mut |test_manager| {
        test_manager.start(app_handle.clone(), Duration::from_millis(interval), already_sent, mode, write_id)
    }))
}

#[tauri::command(async)]
pub fn stop_sending_loop(
    sending_loop_state: tauri::State<'_, SendingLoopState>
) -> Result<(), String> {
    result_to_string(use_struct(&sending_loop_state, &mut |test_manager| {
        test_manager.stop()
    }))
}