use axum::{
    extract::Path,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn root() -> Json<Message> {
    Json(Message {
        message: "Hello from coolify-rust!".to_string(),
    })
}

async fn health() -> Json<Health> {
    Json(Health { status: "ok" })
}

async fn hello(Path(name): Path<String>) -> Json<Message> {
    Json(Message {
        message: format!("Hello, {name}!"),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/hello/:name", get(hello));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
