use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RegisterUserDto {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String
}

pub struct RegisterUserDBEntryDto {
    pub name: String,
    pub email: String,
    pub password: String,
    pub verfication_token: Uuid,
    pub veritication_token_expires_at: DateTime<Utc>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailVerificationDto {
    pub user_id: Uuid,
    pub token: Uuid,
    pub expires_at: DateTime<Utc>
}