use axum::{Router, routing::get};

pub fn user_handler() -> Router {
    Router::new()
        .route("/", get(|| async { "This is get users endpoint".to_string() }))
}