use axum::{Json, Router, extract::State, http::StatusCode, response::{IntoResponse, Response}, routing::post};
use chrono::{Duration, Utc};
use sqlx::{Error, database};
use uuid::Uuid;

use crate::{AppState, database::auth::{AuthExt}, dtos::auth::{RegisterUserDBEntryDto, RegisterUserDto}, models::User};

pub fn auth_handlers() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
}


pub async fn register(State(app_state):State<AppState>, Json(body): Json<RegisterUserDto>) -> impl IntoResponse {
    println!("this is the json body {:?}", body);

    let verification_token = Uuid::new_v4();
    let expires_at = Utc::now() + Duration::hours(24);
    let register_user = RegisterUserDBEntryDto {
        name: body.name,
        email: body.email,
        password: body.password,
        verfication_token: verification_token,
        veritication_token_expires_at: expires_at
    };
    let result = app_state.db_client.create_user(register_user).await;
    
    match result {
        Ok(user) => Json(user).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error").into_response()
    }
}