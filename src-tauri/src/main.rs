#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod usb;

use std::{sync::{Arc, Mutex}, ops::Deref};

use tauri::{Manager, AppHandle, Runtime};

use chrono::Duration;
use usb::UsbManager;

#[tauri::command]
fn greet<R: Runtime>(app: AppHandle<R>, name: &str) -> String {
    app.emit_all("data-received", "TEST").unwrap();

    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn read_from_port(port_name: &str) -> Result<(), String> {
    unsafe {
        match USB_MANAGER.as_mut().unwrap().set_active_port(port_name) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.description),
        }
    }
}

fn send_read_bytes_to_frontend(read_bytes: &[u8]) {
    unsafe {
        APP.as_ref().unwrap().emit_all("data-received", read_bytes).unwrap()
    }
}

// TODO: using a mutable static variable is not great; find a way to satisfy the borrow checker
static mut APP: Option<AppHandle> = None;
static mut USB_MANAGER: Option<UsbManager> = None;

fn main() {
    unsafe {
        USB_MANAGER = Some(UsbManager::new());
    }

    let timer = timer::Timer::new();
    let count = Arc::new(Mutex::new(0));


    timer.schedule_repeating(Duration::seconds(1), move || {
        unsafe {
            let usb_manager = USB_MANAGER.as_mut().unwrap();
            if usb_manager.refresh_available_ports() {
                // Available ports have changed
                APP.as_ref().unwrap().emit_all("ports-changed", &USB_MANAGER.as_ref().unwrap().available_ports).unwrap();
            }

            usb_manager.read_from_active_port(send_read_bytes_to_frontend);
        }

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
        .invoke_handler(tauri::generate_handler![greet, read_from_port])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
