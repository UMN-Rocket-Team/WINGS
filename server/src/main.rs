use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State}, response::IntoResponse
};

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

struct AppState {
    tx: broadcast::Sender<String>
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState{tx});
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/live", get(ws_handler))
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await;

    let mut rx = state.tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if msg == String::new() {
                break;
            }
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    tokio::select! {
        _ = (&mut recv_task) => send_task.abort(),
    };
    
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) {
    ws.on_upgrade(|socket| {handle_socket(socket, state)})
}

fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let _ = state.tx.send("msg");
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}