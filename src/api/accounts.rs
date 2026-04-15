use argon2::{
    Argon2,
    password_hash::{Error, PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::http::StatusCode;
use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::db::models::user::User;

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

async fn register(Json(input): Json<NewUserInput>) -> (StatusCode, Json<Value>) {
    if let Err(e) = pass_check(&input.password, &input.password_confirm) {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": e})));
    }

    if let Err(e) = email_check(&input.email) {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": e})));
    }

    let (password_hash, salt) = match hash_password(&input.password) {
        Ok((hash, salt)) => (hash, salt),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            );
        }
    };

    let new_user = User::new(&input.name, &password_hash, salt.as_str());

    (
        StatusCode::CREATED,
        Json(json!({
            "status": "success",
            "data": {
                "name": new_user.name,
                "hash": new_user.password_hash,
                "salt": new_user.salt,
            }
        })),
    )
}

fn pass_check(password: &str, password_confirm: &str) -> Result<(), &'static str> {
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
            if !local.is_empty() && !domain.starts_with('.') && domain.contains('.') =>
        {
            Ok(())
        }
        _ => Err("Invalid email address"),
    }
}

fn hash_password(password: &str) -> Result<(String, SaltString), Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok((hash, salt))
}
