use axum::{Json, Router, routing::post};
use serde::Deserialize;

#[derive(Deserialize)]
struct NewUserInput {
    name: String,
    email: String,
    password: String,
    password_confirm: String,
}

pub fn accounts_routes() -> Router {
    Router::new().route("/register", post(register))
}

async fn register(Json(input): Json<NewUserInput>) -> String {
    format!("Received data: {} \nPassword confirm {}", input.name,pass_check(&input.password ,&input.password_confirm))
}

fn pass_check(password: &str, password_confirm: &str ) -> bool {
    password == password_confirm
}
