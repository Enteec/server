use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ping", get(ping));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to listen on IP:PORT!");

    axum::serve(listener, app)
        .await
        .expect("Failed to start a server!");
}

async fn ping() -> &'static str {
    "pong"
}
