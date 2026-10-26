use std::sync::Arc;

use axum::{Router, extract::{State, ws::{ CloseFrame, Message, Utf8Bytes, WebSocket, WebSocketUpgrade}}, response::IntoResponse, routing::{any, get} };
use tokio::sync::broadcast;
use futures::{sink::SinkExt, stream::StreamExt};


pub struct AppState {
    tx: broadcast::Sender<String>,
    _rx: broadcast::Receiver<String>
}

pub fn routes() -> Router {
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState{tx, _rx});
    Router::new()
        .route("/live", any(ws_handler))
        .with_state(app_state)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    println!("Upgrading...");
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {

    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut _receiver) = socket.split();
    
    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        let n_msg = 20;
        for i in 0..n_msg {
            // In case of any websocket error, we exit.
            if sender
                .send(Message::Text(Utf8Bytes::from_static("This is a test")))
                .await.is_err()
            {
                return i;
            }

            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }

        println!("Sending close to cient...");
        if let Err(e) = sender
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Utf8Bytes::from_static("Goodbye"),
            })))
            .await
        {
            println!("Could not send Close due to {e}, probably it is ok?");
        }
        n_msg
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
