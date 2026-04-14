use axum::{Router, routing::post};

pub fn accounts_routes() -> Router {
    Router::new().route("/register", post(register))
}

async fn register() -> &'static str {
    "pong"
}
