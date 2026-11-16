use axum::{debug_handler, extract::Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{APP_STATE, config::devices::SerialDevice};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct CommandBody {
    command: String
}

#[debug_handler]
pub async fn send_command(Json(body): Json<CommandBody>) -> StatusCode {
    info!("Recieved command: {}", body.command);
    let mut serial_devices = APP_STATE.serial_devices().await;
    let command_radios = serial_devices.iter_mut().filter(|d| d.is_command()).collect::<Vec<&mut SerialDevice>>();
    match command_radios.len() {
        0 => StatusCode::NO_CONTENT,
        _ => {
            for cr in command_radios {
                let stream = body_command.as_bytes();
            }
            for reciever in APP_STATE.serial_devices().await.iter_mut() {
                if serial_device.is_command() {
                    let stream = body.command.as_bytes();
                    match serial_device.write(stream) {
                        Ok(_) => {
                            info!("Tx({}): {:?}", serial_device.sn(), stream);
                            StatusCode::ACCEPTED
                        },
                        Err(_) => {
                            eprintln!("Failed to tx: {:?}", stream);
                            StatusCode::SERVICE_UNAVAILABLE
                        },
                    }
                }
            }
        }
    }
}