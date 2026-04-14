use axum::{Json, Router, routing::post};
use serde::Deserialize;

#[derive(Deserialize)]
struct NewUserInput {
    name: String,
}

pub fn accounts_routes() -> Router {
    Router::new().route("/register", post(register))
}

async fn register(Json(input): Json<NewUserInput>) -> String {
    format!("Received data: {}", input.name)
}
