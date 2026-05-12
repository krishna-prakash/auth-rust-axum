use axum::{Json, Router, extract::State, http::StatusCode, response::{IntoResponse, Response}, routing::post};
use chrono::{Duration, Utc};
use sqlx::{Error, database};
use uuid::Uuid;

use crate::{AppState, database::auth::AuthExt, dtos::auth::{RegisterUserDBEntryDto, RegisterUserDto}, mail::mail::send_verification_email, models::User, utils::password};

pub fn auth_handlers() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
}


pub async fn register(
    State(app_state):State<AppState>,
    Json(body): Json<RegisterUserDto>
) -> Result<Response, Response> {
    println!("this is the json body {:?}", body);

    let verification_token = Uuid::new_v4();
    let expires_at = Utc::now() + Duration::hours(24);
    let hash_password = password::hash(&body.password)
        .map_err(|e| e.into_response())?;
    let register_user = RegisterUserDBEntryDto {
        name: body.name,
        email: body.email,
        password: hash_password,
        verfication_token: verification_token,
        veritication_token_expires_at: expires_at
    };
    let result = app_state.db_client.create_user(register_user).await;

    match result {
        Ok(user) => {
        let em = send_verification_email(
            &app_state.mail_config,
            &user.email,
            verification_token
        ).await;

        
        if let Err(_e) = em {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "We are unable to send verification email, please login and resend manually"
            ).into_response());
        };

        match em {
            Ok(_) => println!("email sent successfully"),
            Err(e) => println!("{:?}", e.to_string())
        }
        
        Ok((
           StatusCode::CREATED,
           Json(user)   
        ).into_response())
    },
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string()
        ).into_response())
    } 
}