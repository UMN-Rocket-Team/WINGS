use axum::{debug_handler, extract::Json};
use serde::{Deserialize, Serialize};

use crate::APP_STATE;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct CommandBody {
    command: String
}

#[debug_handler]
pub async fn send_command(Json(body): Json<CommandBody>) {
    println!("Recieved command: {}", body.command);
    for serial_device in APP_STATE.serial_devices().await.iter_mut() {
        if serial_device.is_command() {
            let stream = body.command.as_bytes();
            match serial_device.write(stream) {
                Ok(_) => println!("Tx({}): {:?}", serial_device.sn(), stream),
                Err(_) => eprintln!("Failed to tx: {:?}", stream),
            }
        }
    }
}