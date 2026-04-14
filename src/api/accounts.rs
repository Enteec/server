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
   // format!("Received data: {} \nPassword confirm {}\nEmail confirm {}", input.name,pass_check(&input.password ,&input.password_confirm),email_check(&input.email))
    "Test1".to_string()
}

fn pass_check(password: &str, password_confirm: &str ) -> Result<(), &'static str> {
    if password != password_confirm {
        return Err("Passwords do not match");
    }

    if password.len() < 8 {
        return Err("Password too short");
    }

    if !password.chars().any(|c| c.is_uppercase()) {
        return Err("Missing uppercase letter");
    }

    if !password.chars().any(|c| !c.is_alphanumeric()) {
        return Err("Missing special character");
    }

    Ok(())
}

fn email_check(email: &str) -> Result<(), &'static str> {
    if email.contains(char::is_whitespace) {
        return Err("Email contains whitespace");
    }

    if email.starts_with('@') || email.starts_with('.') {
        return Err("Email contains an invalid character");
    }

    if email.ends_with('@') || email.ends_with('.') {
        return Err("Email contains an invalid character");
    }

    let mut parts = email.split('@');

    match (parts.next(), parts.next(), parts.next()) {
        (Some(local), Some(domain), None)
        if !local.is_empty() && !domain.starts_with('.') && domain.contains('.') => Ok(()),
        _ => Err("Invalid email address"),
    }
}