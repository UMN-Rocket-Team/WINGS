//! # Sending Commands
//!
//! This module defines the Tauri commands that control the background data sending loop.
//!
//! The commands act as a bridge, receiving requests from the frontend and interacting
//! with the shared `SendingLoopState` to manage the lifecycle of the sending task.

use std::time::Duration;

use crate::{
    sending_loop::{SendingLoopState, SendingModes},
    state::{generic_state::result_to_string, mutex_utils::use_state_in_mutex},
};

/// Starts the background data sending loop.
///
/// # Arguments
/// * `app_handle` - The Tauri application handle, used to access shared state like configuration.
/// * `sending_loop_state` - The shared state containing the `SendingLoop` manager.
/// * `interval` - The time to wait between sending each packet, in milliseconds.
/// * `already_sent` - An initial count for the number of packets sent, useful for resuming.
/// * `mode` - The `SendingModes` enum that dictates how packet data is generated (e.g., from a CSV, all zeros, etc.).
/// * `write_id` - The identifier for the communication port/channel to which the data will be written.
///
/// # Returns
/// * `Result<(), String>` - An empty `Ok` on success, or an `Err` with a descriptive string if the loop fails to start.
#[tauri::command(async)]
pub fn start_sending_loop(
    app_handle: tauri::AppHandle,
    sending_loop_state: tauri::State<'_, SendingLoopState>,
    interval: u64,
    already_sent: u32,
    mode: SendingModes,
    write_id: usize,
) -> Result<(), String> {
    result_to_string(use_state_in_mutex(
        &sending_loop_state,
        &mut |test_manager| {
            test_manager.start(
                app_handle.clone(),
                Duration::from_millis(interval),
                already_sent,
                mode,
                write_id,
            )
        },
    ))
}

/// Stops the background data sending loop.
///
/// # Arguments
/// * `sending_loop_state` - The shared state containing the `SendingLoop` manager.
///
/// # Returns
/// * `Result<(), String>` - An empty `Ok` on success, or an `Err` with a descriptive string if the loop fails to stop.
#[tauri::command(async)]
pub fn stop_sending_loop(
    sending_loop_state: tauri::State<'_, SendingLoopState>,
) -> Result<(), String> {
    // Safely access the SendingLoop within the Mutex and call its stop method.
    result_to_string(use_state_in_mutex(
        &sending_loop_state,
        &mut |test_manager| test_manager.stop(),
    ))
}
