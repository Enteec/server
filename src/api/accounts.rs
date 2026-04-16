use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{AppState, db::models::user::User, errors::RegisterError};

#[derive(Deserialize)]
struct NewUserInput {
    name: String,
    email: String,
    password: String,
    password_confirm: String,
}

pub fn accounts_routes() -> Router<AppState> {
    Router::new().route("/register", post(register))
}

async fn register(
    State(state): State<AppState>,
    Json(input): Json<NewUserInput>,
) -> Result<(StatusCode, Json<Value>), RegisterError> {
    pass_check(&input.password, &input.password_confirm)?;
    email_check(&input.email)?;

    let (password_hash, salt) = hash_password(&input.password)?;

    let new_user = User::new(&input.name, &input.email, &password_hash, salt.as_str());

    let conn = &mut state.db_pool.get()?;

    User::create(&new_user, conn).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "success",
            "data": {
                "message": "User registered"
            }
        })),
    ))
}

fn pass_check(password: &str, password_confirm: &str) -> Result<(), RegisterError> {
    if password != password_confirm {
        return Err(RegisterError::PasswordsDontMatch);
    }

    if password.len() < 8 {
        return Err(RegisterError::WeakPassword);
    }

    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(RegisterError::WeakPassword);
    }

    if !password.chars().any(|c| !c.is_alphanumeric()) {
        return Err(RegisterError::WeakPassword);
    }

    Ok(())
}

fn email_check(email: &str) -> Result<(), RegisterError> {
    if email.contains(char::is_whitespace) {
        return Err(RegisterError::InvalidEmail);
    }

    if email.starts_with('@') || email.starts_with('.') {
        return Err(RegisterError::InvalidEmail);
    }

    if email.ends_with('@') || email.ends_with('.') {
        return Err(RegisterError::InvalidEmail);
    }

    let mut parts = email.split('@');

    match (parts.next(), parts.next(), parts.next()) {
        (Some(local), Some(domain), None)
            if !local.is_empty() && !domain.starts_with('.') && domain.contains('.') =>
        {
            Ok(())
        }
        _ => Err(RegisterError::InvalidEmail),
    }
}

fn hash_password(password: &str) -> Result<(String, SaltString), RegisterError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok((hash, salt))
}
