use std::{cell::OnceCell, path::PathBuf, str::FromStr, sync::Mutex};

use axum::{Router, routing::get};

use serialport::{SerialPortInfo, SerialPortType};
use wings_packet_parser::{drivers::serial, prelude::*};

pub use tokio::main;

use crate::config::Config;

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
                None => return Err(serialport::Error { 
                    kind: serialport::ErrorKind::Unknown, 
                    description: String::from("serial port serial_number does not exist") 
                })
            };

            
            Ok(
                DeviceName {
                    name: device.port_name,
                    value: serial_number,
                    manufacturer_name: usb.manufacturer,
                    product_name: usb.product
                }
            )
        },
        _ => Err(serialport::Error { 
            kind: serialport::ErrorKind::NoDevice,
            description: format!("failed to match device of type {:?}",
            device.port_type) 
        })
    } 
    

}

fn match_config_devices() -> Result<Vec<Box<dyn serialport::SerialPort>>, serialport::Error> {
    let matched_devices = vec![];
    match serialport::available_ports()  {
        Ok(devices) => {
            for device in devices {
                /* 
                        Don't want to use options because it doesn't provide details _why_ something failed.
                        Currently defining a DeviceConfigError that gives some information about why
                        reading from a serial port failed. Result gives us some extensibility in terms of handling failed 
                        serial port reading(s). silently failing here 
                */
               
                match construct_device_name(device) {
                    Ok(device) => {
                        // do some matching w/ config file

                    },
                    Err(e) => {
          
                        continue;
                    }
                } 
            }
            Ok(matched_devices) // returns a list of our selected devices in our config file
        },
        Err(e) => Err(e),
    }
}

#[tokio::main]
async fn main() {
    let config_path = PathBuf::from_str(CONFIG_PATH).unwrap();
    let config = config::Config::load_or_default(&config_path);
    let _ = match CONFIG.lock() {
        Ok(l) => l.set(config.to_owned()),
        Err(e) => panic!("Could not initialize config state: {}", e.to_string()),
    };

    match_config_devices();
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


