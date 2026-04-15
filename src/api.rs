mod accounts;

use crate::{AppState, api::accounts::accounts_routes};
use axum::Router;

pub fn api_routes() -> Router<AppState> {
    Router::new().nest("/accounts", accounts_routes())
}
