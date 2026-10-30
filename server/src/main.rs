use std::{cell::OnceCell, path::PathBuf, str::FromStr, sync::Mutex};

use axum::{Router, routing::get};

use wings_packet_parser::prelude::*;

pub use tokio::main;

use crate::config::Config;

pub mod config;
pub mod uart;

pub const CONFIG_PATH: &str = "./config.toml";

pub static PS_MANAGER: Mutex<OnceCell<PacketStructureManager>> =
    Mutex::<OnceCell<PacketStructureManager>>::new(OnceCell::new());
pub static CONFIG: Mutex<OnceCell<Config>> = Mutex::<OnceCell<Config>>::new(OnceCell::new());

#[tokio::main]
async fn main() {
    let config_path = PathBuf::from_str(CONFIG_PATH).unwrap();
    let config = config::Config::load_or_default(&config_path);
    let _ = match CONFIG.lock() {
        Ok(l) => l.set(config.to_owned()),
        Err(e) => panic!("Could not initialize config state: {}", e.to_string()),
    };

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
