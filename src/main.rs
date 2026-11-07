use tokio::net::{TcpListener};
use std::env;

use axum::{Router, routing::get, response::Html};

async fn index() -> Html<String> {
    let websitefile = concat!(env!("CARGO_MANIFEST_DIR"), "/website/index.html");

    Html(tokio::fs::read_to_string(websitefile).await.unwrap())
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(index));
    let tcpconnection = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    axum::serve(tcpconnection, router).await.unwrap();
}
