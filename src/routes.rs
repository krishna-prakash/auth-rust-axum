use axum::{Router, routing::get};

fn user_handler() -> Router {
    Router::new()
        .route("/", get(|| async { "This is get users endpoint".to_string() }))
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "hello world".to_string() }))
        .nest("/user", user_handler())
}

