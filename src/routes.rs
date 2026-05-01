use axum::{Router, routing::get};

use crate::handlers;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "hello world".to_string() }))
        .nest("/user", handlers::users::user_handler())
}

