mod api;
mod db;
mod errors;

use crate::{
    api::api_routes,
    db::{DbPool, create_pool},
};
use axum::Router;

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
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to listen on IP:PORT!");

    axum::serve(listener, app)
        .await
        .expect("Failed to start a server!");
}
