mod api;
mod db;

use crate::{api::api_routes, db::establish_connection};
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let _connection = &mut establish_connection();

    let app = Router::new()
        .nest("/api", api_routes())
        .route("/ping", get(ping));

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
