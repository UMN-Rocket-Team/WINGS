use axum::{debug_handler, extract::Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::{APP_STATE, config::devices::SerialDevice};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct CommandBody {
    command: String
}

#[debug_handler]
pub async fn send_command(Json(body): Json<CommandBody>) -> (StatusCode, Json<Vec<String>>) {
    info!("Recieved command: {}", body.command);
    let mut serial_devices = APP_STATE.serial_devices().await;
    let command_radios = serial_devices.iter_mut().filter(|d| d.is_command()).collect::<Vec<&mut SerialDevice>>();
    let count = command_radios.len();
    match count {
        0 => (StatusCode::NO_CONTENT, Json(vec![])),
        _ => {
            let mut failed_sends: Vec<String> = vec![];
            for cr in command_radios {
                let stream = body.command.as_bytes();
                match cr.write(stream) {
                    Ok(_) => {
                        info!("Tx({}): {:?}", cr.sn(), stream);
                    },
                    Err(_) => {
                        error!("Failed to tx: {:?}", stream);
                        failed_sends.push(format!("{}:{}", cr.sn(), body.command));
                    }
                }
            }
            if failed_sends.len() != 0 {
                if failed_sends.len() == count {
                    return (StatusCode::SERVICE_UNAVAILABLE, Json(failed_sends))
                }
                return (StatusCode::PARTIAL_CONTENT, Json(failed_sends))
            }
            (StatusCode::ACCEPTED, Json(vec![]))
        }
    }
}