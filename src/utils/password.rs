use std::sync::LazyLock;

use argon2::{
    Argon2, 
    password_hash::{ Error as PasswordHashError }
    , PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};

const MAX_PASSWORD_LENGTH: usize = 64;
const MIN_PASSWORD_LENGTH: usize = 8;

static ARGON2: LazyLock<Argon2<'static>> = LazyLock::new(|| { 
    Argon2::default()
});

pub fn hash(password: impl AsRef<[u8]>) -> Result<String, &'static str> {
    let password = password.as_ref();
    let salt = SaltString::generate(&mut OsRng);
    let hash = ARGON2.hash_password(password, &salt).map_err(|e| match e { 
        PasswordHashError::Password => "Hash error",
        _ => "generic error"
     })?;
    Ok(hash.to_string())
}