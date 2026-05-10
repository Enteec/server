use argon2::password_hash::Error as ArgonError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use diesel::result::Error as DieselError;
use r2d2::Error as R2d2Error;
use serde_json::json;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum RegisterError {
    UserExists,
    WeakPassword,
    PasswordsDontMatch,
    InvalidEmail,
    HashError(ArgonError),
    DbConnectionError(R2d2Error),
    DbError(DieselError),
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        let (code, message) = match self {
            RegisterError::UserExists => (
                StatusCode::BAD_REQUEST,
                "User with this name already exists",
            ),
            RegisterError::WeakPassword => (StatusCode::BAD_REQUEST, "Weak password"),
            RegisterError::PasswordsDontMatch => {
                (StatusCode::BAD_REQUEST, "Passwords do not match")
            }
            RegisterError::InvalidEmail => (StatusCode::BAD_REQUEST, "Invalid email"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({"error": message}));

        (code, body).into_response()
    }
}

impl Display for RegisterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            RegisterError::UserExists => write!(f, "user already exists"),
            RegisterError::WeakPassword => write!(f, "password is too weak"),
            RegisterError::PasswordsDontMatch => write!(f, "passwords do not match"),
            RegisterError::InvalidEmail => write!(f, "invalid email address"),

            RegisterError::HashError(err) => {
                write!(f, "{}", err)
            }

            RegisterError::DbConnectionError(err) => {
                write!(f, "{}", err)
            }

            RegisterError::DbError(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

impl From<ArgonError> for RegisterError {
    fn from(err: ArgonError) -> Self {
        RegisterError::HashError(err)
    }
}

impl From<R2d2Error> for RegisterError {
    fn from(err: R2d2Error) -> Self {
        RegisterError::DbConnectionError(err)
    }
}

impl From<DieselError> for RegisterError {
    fn from(err: DieselError) -> Self {
        RegisterError::DbError(err)
    }
}
