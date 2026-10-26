use axum::{
    routing::get,
    Router,
};

mod controllers;



#[tokio::main]
async fn main() {

    
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .nest("/ws", controllers::ws_controller::routes());
        
    
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    
    match axum::serve(listener, app).await {
        Err(_) => println!("failed"),
        _ => println!("succeded")
    };
    
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

