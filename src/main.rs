use std::net::TcpListener;

use axum;

fn main() {
    let tcpconnection = TcpListener::bind("127.8.3.1", 8080);
}
