use std::sync::Arc;

use axum::{
    Router,
    extract::{
        State,
        ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
    routing::{any, get},
};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;

pub struct AppState {
    tx: broadcast::Sender<String>,
    _rx: broadcast::Receiver<String>,
}

pub fn routes() -> Router {
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState { tx, _rx });
    Router::new()
        .route("/live", any(ws_handler))
        .with_state(app_state)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    println!("Upgrading...");
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn generate_string() -> Utf8Bytes {
    tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
    Utf8Bytes::from_static("This is my websocket endpoint!")
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut _receiver) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        /*
        some async stuff that we will hook up later
        while let msg = generate_string().await {
            if msg == String::new() {
                break;
            }
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
         */

        for i in 0..20 {
            if sender
                .send(Message::Text(Utf8Bytes::from(format!("{}", i))))
                .await
                .is_err()
            {
                break;
            };
            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }

        "finished"
    });

    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => println!("{a} messages sent to cient"),
                Err(a) => println!("Error sending messages {a:?}")
            }
        },

    }

    // returning from the handler closes the websocket connection
    println!("Websocket context cient destroyed");
}
