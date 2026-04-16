use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use argon2::password_hash::Error as ArgonError;
use diesel::result::Error as DieselError;
use r2d2::Error as R2d2Error;

pub enum RegisterError {
    WeakPassword,
    PasswordsDontMatch,
    InvalidEmail,
    HashError,
    DbConnectionError,
    DbError,
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        let (code, message) = match self {
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

impl From<ArgonError> for RegisterError {
    fn from(_: ArgonError) -> Self {
        RegisterError::HashError
    }
}

impl From<R2d2Error> for RegisterError {
    fn from(_: R2d2Error) -> Self {
        RegisterError::DbConnectionError
    }
}

impl From<DieselError> for RegisterError {
    fn from(_: DieselError) -> Self {
        RegisterError::DbError
    }
}
