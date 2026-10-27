use std::{cell::OnceCell, sync::Mutex};

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use wings_packet_parser::prelude::*;

pub use tokio::main;

pub mod uart;

pub const PS_MANAGER: OnceCell<Mutex<PacketStructureManager>> =
    OnceCell::<Mutex<PacketStructureManager>>::new();

#[tokio::main]
async fn main() {
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
