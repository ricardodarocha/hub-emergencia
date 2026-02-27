use argon2::{Argon2, PasswordHash, PasswordVerifier};
use password_hash::SaltString; 
use thiserror::Error;
use argon2::PasswordHasher;
use axum::http::StatusCode;
use axum::response::{Response, IntoResponse};
use rand::rngs::OsRng;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("password hashing failed")]
    HashError(#[from] argon2::password_hash::Error),
}

impl IntoResponse for PasswordError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            self.to_string(),
        ).into_response()
    }
}

pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    let salt = SaltString::generate(&mut OsRng);

    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(h) => h,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}