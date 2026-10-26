use std::sync::Arc;

use axum::{Router, extract::{State, ws::{ Message, WebSocket, WebSocketUpgrade}}, response::IntoResponse, routing::get };
use tokio::sync::broadcast;
use futures::{sink::SinkExt, stream::StreamExt};


pub struct AppState {
    tx: broadcast::Sender<String>
}

pub fn routes() -> Router {
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState{tx});
    Router::new()
        .route("/live", get(ws_handler))
        .with_state(app_state)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| {handle_socket(socket, state)})
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, _) = socket.split();
    

    let mut rx = state.tx.subscribe();
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await { // I think we have to be receiving packets here
            // probably async transmission of packets via packetparser library here
            
            // what's the difference between the tx stored in AppState and the ssend channel received from .split()?
            
            // then continuing to put the packets forward here.
            sender.send(Message::text(msg));
        }
    });



    
}