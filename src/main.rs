mod api;
mod db;
mod errors;

use crate::{
    api::api_routes,
    db::{DbPool, create_pool},
};

use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::any,
};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
}

#[tokio::main]
async fn main() {
    let pool = create_pool();

    let app_state = AppState { db_pool: pool };

    let app = Router::new()
        .nest("/api", api_routes())
        .route("/ws", any(ws_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to listen on IP:PORT!");

    axum::serve(listener, app)
        .await
        .expect("Failed to start a server!");
}

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                println!("Received: {}", text);
                if socket
                    .send(Message::Text("Hello from server!".into()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => (),
        }
    }
}
