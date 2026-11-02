use std::{cell::OnceCell, path::PathBuf, str::FromStr, sync::Mutex};

use axum::{Router, routing::get};

use serialport::{SerialPortInfo, SerialPortType};
use tracing::error;
use wings_packet_parser::{drivers::serial, prelude::*};

pub use tokio::main;

use crate::config::{Config, RadioOptions};

pub mod config;
pub mod uart;

pub const CONFIG_PATH: &str = "./config.toml";

pub static PS_MANAGER: Mutex<OnceCell<PacketStructureManager>> =
    Mutex::<OnceCell<PacketStructureManager>>::new(OnceCell::new());
pub static CONFIG: Mutex<OnceCell<Config>> = Mutex::<OnceCell<Config>>::new(OnceCell::new());

fn construct_device_name(device: SerialPortInfo) -> Result<DeviceName, serialport::Error> {
    println!("device_name: {}", device.port_name);
    match device.port_type {
        SerialPortType::UsbPort(usb) => {
            let serial_number = match usb.serial_number {
                Some(s) => s,
                None => {
                    return Err(serialport::Error {
                        kind: serialport::ErrorKind::Unknown,
                        description: String::from("serial port serial_number does not exist"),
                    });
                }
            };

            Ok(DeviceName {
                name: serial_number,
                value: device.port_name,
                manufacturer_name: usb.manufacturer,
                product_name: usb.product,
            })
        }
        _ => Err(serialport::Error {
            kind: serialport::ErrorKind::NoDevice,
            description: format!("failed to match device of type {:?}", device.port_type),
        }),
    }
}

fn match_radio_devices(
    config: &RadioOptions,
) -> Result<Vec<Box<dyn serialport::SerialPort>>, serialport::Error> {
    let mut matched_devices = vec![];
    let devices = serialport::available_ports();

    if devices.is_err() {
        return Err(serialport::Error {
            kind: serialport::ErrorKind::NoDevice,
            description: String::from("unable to list available ports"),
        });
    }

    for device in devices.unwrap() {
        match construct_device_name(device) {
            Ok(device) => {
                // do some matching w/ config file
                if let Some(transmitter) = config.transmitter()
                    && device == *transmitter
                {
                    match serialport::new(transmitter.value.clone(), 50).open() {
                        Ok(dev) => matched_devices.push(dev),
                        Err(e) => error!("failed to open device port: {}", e)
                    }
                }
                if let Some(receiver) = config.receiver()
                    && device == *receiver
                {
                    match serialport::new(receiver.value.clone(), 50).open() {
                        Ok(dev) => matched_devices.push(dev),
                        Err(e) => error!("failed to open device port: {}", e)
                    }
       
                            
                }
            }
            Err(e) => {
                error!("failed to make device: {}", e)
            }
        }
    }

    Ok(matched_devices) // returns a list of our selected devices in our config file
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let config_path = PathBuf::from_str(CONFIG_PATH).unwrap();
    let config = config::Config::load_or_default(&config_path);
    let _ = match CONFIG.lock() {
        Ok(l) => l.set(config.to_owned()),
        Err(e) => panic!("Could not initialize config state: {}", e.to_string()),
    };

    let mut devices = vec![];

    if let Some(radio_options) = config.radio_options() {
        match match_radio_devices(radio_options) {
            Ok(res) => devices = res,
            Err(e) => error!("failed to initialize radio(s): {}", e),
        };
    }
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await;
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
