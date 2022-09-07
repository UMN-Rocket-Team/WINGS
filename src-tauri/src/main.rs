#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{sync::{Arc, Mutex}, ops::Deref};

use tauri::{Manager, AppHandle, Runtime};

use chrono::Duration;

#[tauri::command]
fn greet<R: Runtime>(app: AppHandle<R>, name: &str) -> String {
    app.emit_all("data-received", "TEST").unwrap();

    format!("Hello, {}! You've been greeted from Rust!", name)
}

// TODO: using a mutable static variable is not great; find a way to satisfy the borrow checker
static mut APP: Option<AppHandle> = None;

fn main() {
    let timer = timer::Timer::new();
    let count = Arc::new(Mutex::new(0));

    timer.schedule_repeating(Duration::seconds(1), move || {
        let mut count_ref = count.lock().unwrap();
        *count_ref += 1;
        
        unsafe {
            APP.as_ref().unwrap().emit_all("data-received", count_ref.deref()).unwrap();
        }
    }).ignore();
    
    tauri::Builder::default()
        .setup(move |app| {
            unsafe {
                APP = Some((&app).app_handle());
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
