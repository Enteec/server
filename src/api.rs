mod accounts;

use crate::api::accounts::accounts_routes;
use axum::Router;

pub fn api_routes() -> Router {
    Router::new().nest("/accounts", accounts_routes())
}
