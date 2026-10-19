


pub fn routes() -> Router {
    Router::new().route("/live", get(ws_handler))
}

fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) {
    ws.on_upgrade(|socket| {handle_socket(socket, state)})
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    
    let msg = Message::string("Opening packet stream from server...");
    while let Ok(msg) = state.tx.send(msg).await {
        // probably async transmission of packets via packetparser library here
    }
}