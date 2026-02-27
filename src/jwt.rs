use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{Utc, Duration};

const SECRET: &[u8] = b"23aacf19-d4ed-5fc4-a1ad-db0dbc9d882c-8962a11f-7fff-5b5a-92cb-cc9275dbb447";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

// use thiserror::Error;
// #[derive(Debug, Error)]
// pub enum JwtError {
//     #[error("token creation failed")]
//     Creation(#[from] jsonwebtoken::errors::Error),

//     #[error("token validation failed")]
//     Validation(#[from] jsonwebtoken::errors::Error),
// }

pub type JwtResult<T> = Result<T, jsonwebtoken::errors::Error>;

pub fn generate(user_id: Uuid) -> JwtResult<String> {
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims { sub: user_id, exp };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )?;  

    Ok(token)
}

pub fn validate(token: &str) -> JwtResult<Claims> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )?;

    Ok(data.claims) 
}