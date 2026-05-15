use axum::{Json, Router, extract::{Query, State}, http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}};
use chrono::{Duration, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{AppState, database::auth::AuthExt, dtos::auth::{RegisterUserDBEntryDto, RegisterUserDto}, mail::mail::{send_verification_email, send_welcome_email}, models::User, utils::password};

pub fn auth_handlers() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/verify", get(verify))
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

#[derive(Deserialize)]
pub struct VerifyParams {
    token: String
}

pub async fn verify(
    State(app_state): State<AppState>,
    Query(params): Query<VerifyParams> 
) -> Result<Response, Response> {

    let token = Uuid::parse_str(&params.token)
        .map_err(|e| e.to_string().into_response())?;

    let result = app_state
        .db_client
        .get_user_id_by_token(token)
        .await
        .map_err(|e| e.to_string().into_response())?;

    let email_verification = result
        .ok_or_else(|| ((
            StatusCode::BAD_REQUEST,
            "invalid token"
        )).into_response())?;

    if Utc::now() > email_verification.expires_at {
        return Err((
            StatusCode::BAD_REQUEST,
            "token_expired"
        ).into_response())
    }
    
    app_state
        .db_client
        .verify_user(email_verification.user_id)
        .await
        .map_err(|e| e.to_string().into_response())?;

    let user = app_state
        .db_client
        .get_user(Some(email_verification.user_id), None, None)
        .await
        .map_err(|e| e.to_string().into_response())?;

    let user = user.ok_or_else(|| ((
        StatusCode::BAD_REQUEST,
        "invalid token"
    )).into_response())?;

    let send_welcome_email = send_welcome_email(
        &app_state.mail_config, &user.email
    ).await;

    if let Err(e) = send_welcome_email {
        eprint!("Failed to send welcome email {}", e);
    }
    // send welcome email
    Ok((
        StatusCode::OK,
        "verification done"
    ).into_response())
}