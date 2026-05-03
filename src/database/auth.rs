use sqlx::Error;

use crate::{database::DBClient, dtos::auth::RegisterUserDBEntryDto, models::User};

pub trait AuthExt {
    async fn create_user(
        &self,
        register_user_info: RegisterUserDBEntryDto
    ) -> Result<User, Error>;
}

impl AuthExt for DBClient {
    async  fn create_user(
            &self,
            register_user_info: RegisterUserDBEntryDto
        ) -> Result<User, Error> {
            let name = register_user_info.name.to_string();
            let email = register_user_info.email.to_string();
            let password = register_user_info.password.to_string();
            let verification_token = register_user_info.verfication_token;
            let mut tx = self.pool.begin().await?;

            let user = sqlx::query_as!(
                User,
                r#"
               INSERT INTO USERS (name, email, password)
               VALUES ($1, $2, $3)
               RETURNING id, name, email, password, email_verified, pending_email, pending_email_token, pending_email_expires_at, created_at, updated_at 
                "#,
                name,
                email,
                password
            ).fetch_one(&mut *tx)
            .await?;

        sqlx::query!(
            r#"
                INSERT INTO email_verifications(user_id, token, expires_at)
                VALUES($1, $2::UUID, $3)
            "#,
            user.id,
            verification_token,
            register_user_info.veritication_token_expires_at
        ).execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(user)
        
    } 
}