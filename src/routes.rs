use axum::{Router, routing::get};

use crate::{AppState, handlers};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "hello world".to_string() }))
        .nest("/user", handlers::users::user_handler())
        .nest("/auth", handlers::auth::auth_handlers())
}

