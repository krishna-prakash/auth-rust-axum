use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::types::Uuid;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub email_verified: Option<bool>,
    pub pending_email:  Option<String>,
    pub pending_email_token: Option<Uuid>,
    pub pending_email_expires_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

