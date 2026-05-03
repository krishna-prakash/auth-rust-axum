use axum::{Router, routing::get};

use crate::AppState;

pub fn user_handler() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "This is get users endpoint".to_string() }))
}