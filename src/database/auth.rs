use sqlx::Error;
use uuid::Uuid;

use crate::{database::DBClient, dtos::auth::{EmailVerificationDto, RegisterUserDBEntryDto}, models::User, utils::password};

pub trait AuthExt {
    async fn create_user(
        &self,
        register_user_info: RegisterUserDBEntryDto
    ) -> Result<User, Error>;
    async fn get_user_id_by_token(
        &self,
        token: Uuid
    ) -> Result<Option<EmailVerificationDto>, Error>;
    async fn verify_user(
        &self,
        user_id: Uuid,
    ) -> Result<(), Error>;
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, Error>;
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

    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, Error>
    {
        let mut query = String::from("SELECT * FROM users WHERE 1=1");
        if let Some(_user_id) = user_id {
            query.push_str("AND id=$1");
        }
        if let Some(_name) = name {
            query.push_str("AND name=$2");
        }
        if let Some(_email) = email {
            query.push_str("AND id=$1");
        }

        let row = sqlx::query_as::<_, User>(&query)
            .bind(user_id)
            .bind(name)
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row)
    }

    async fn get_user_id_by_token(&self, token: Uuid) -> Result<Option<EmailVerificationDto>, Error> {
        let record = sqlx::query_as!(
            EmailVerificationDto,
            r#"
                SELECT user_id, token, expires_at
                FROM email_verifications
                WHERE token = $1
            "#,
            token
        ).fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn verify_user(&self, user_id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            r#"
                UPDATE users
                SET email_verified = true
                WHERE id = $1
            "#,
            user_id
        ).execute(&self.pool)
        .await?;

        sqlx::query!(
            r#"
                DELETE FROM email_verifications
                WHERE user_id = $1
            "#,
            user_id
        ).execute(&self.pool)
        .await?;

        Ok(())
    }
}