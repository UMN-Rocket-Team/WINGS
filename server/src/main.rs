use std::{
    ops::Deref,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, OnceLock},
};

use axum::{Json, Router, debug_handler, routing::get};

use tokio::sync::{Mutex, MutexGuard};
use wings_packet_parser::prelude::*;

pub use tokio::main;

use crate::config::Config;

pub mod config;
pub mod controllers;
pub mod uart;

pub const CONFIG_PATH: &str = "./config.toml";
pub const LOG_PATH: &str = "./log";

pub struct State {
    communication_manager: Mutex<CommunicationManager>,
    config: Mutex<Config>,
    data_processor: Mutex<DataProcessor>,
    log_handler: Mutex<LogHandler>,
}

pub(crate) struct AppState(OnceLock<State>);

impl AppState {
    pub async fn config(&self) -> MutexGuard<'_, Config> {
        (*self).config.lock().await
    }
    pub async fn communication_manager(&self) -> MutexGuard<'_, CommunicationManager> {
        (*self).communication_manager.lock().await
    }
    pub async fn data_processor(&self) -> MutexGuard<'_, DataProcessor> {
        (*self).data_processor.lock().await
    }
    pub async fn log_handler(&self) -> MutexGuard<'_, LogHandler> {
        (*self).log_handler.lock().await
    }
}

impl Deref for AppState {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        self.0.wait()
    }
}

pub(crate) static APP_STATE: AppState = AppState(OnceLock::<State>::new());

#[tokio::main]
async fn main() {
    let config_path = PathBuf::from_str(CONFIG_PATH).unwrap();
    let file_config = config::Config::load_or_default(&config_path);
    let port = file_config.port();
    let config = Mutex::new(file_config);
    let ps_manager = Arc::<Mutex<PacketStructureManager>>::default();
    let communication_manager =
        Mutex::new(CommunicationManager::default_state_tokio(&ps_manager).await);
    let data_processor = Mutex::new(DataProcessor::default_state_tokio(&ps_manager).await);
    let log_handler = Mutex::new(LogHandler::new(PathBuf::new()));

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/config", get(get_config))
        .route("/devices", get(devices))
        .nest("/ws", controllers::ws_controller::routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    let state = State {
        config,
        communication_manager,
        data_processor,
        log_handler,
    };

    let _ = APP_STATE.0.set(state.into());

    let _ = axum::serve(listener, app).await;
}

#[debug_handler]
async fn get_config() -> Json<Config> {
    Json(APP_STATE.config().await.clone())
}

#[debug_handler]
async fn devices() -> Json<Option<Vec<DeviceName>>> {
    Json(
        APP_STATE
            .communication_manager()
            .await
            .get_all_potential_devices(),
    )
}
