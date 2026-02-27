use axum::{
    extract::State,
    // routing::post,
    Json,
    // Router,
};
use serde::{Deserialize, Serialize};
// use uuid::Uuid;

use crate::handlers::AppState;
use crate::models::User;
use crate::jwt;

//modulo especifico para implementar hash de senha segura
use crate::password::verify_password;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

 pub async fn generate_token(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, axum::http::StatusCode> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, name, email, password FROM users WHERE email = $1"#,
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = match user {
        Some(u) => u,
        None => {
            const DUMMY_HASH: &str = "kaioqweq:9201312qiw3120io32oi424io2oi.rustuptoolchainsnightly-x86_64-pc-windows-msvclibrustlibsrcrustlibraryallocsrcstring.rs:2868";
            let _ = verify_password(&payload.password, DUMMY_HASH);     
            return Err(axum::http::StatusCode::UNAUTHORIZED)
        },    
    };

    // 🔐 verificação segura
    let password_valid = verify_password(&payload.password, &user.password);
    if !password_valid {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }

    let token = jwt::generate(user.id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {token}))
}
