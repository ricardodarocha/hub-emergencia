use axum::{
    extract::{State},
    http::{Request, StatusCode},
    response::Response,
    middleware::Next, 
};
 
// use headers::{Authorization, authorization::Bearer};
use axum::body::Body;
use tracing::{error, info}; 
// use uuid::Uuid;

use crate::handlers::AppState; 
use crate::jwt;

#[derive(Clone, PartialEq)]
pub enum UserKind {
    User,
    App
}

impl std::fmt::Display for UserKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserKind::User => write!(f, "User"),
            UserKind::App => write!(f, "App"),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct CurrentUser {
    pub user_id: String,
    pub kind: UserKind,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {

    // 🔹 Copia para String imediatamente (encerra borrow)
    let emergency_id = req
        .headers()
        .get("x-emergencial-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    if let Some(app_id) = emergency_id {         
        // info!("{}", &app_id);
        let exists: Option<i32> =
        sqlx::query_scalar("SELECT 1 FROM app WHERE uid = $1 LIMIT 1")
            .bind(&app_id)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| {
                error!("Database error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
         
        if exists.is_none() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        // 🔹 insere usuário autenticado
        req.extensions_mut().insert(CurrentUser {
            user_id: app_id,
            kind: UserKind::App,
        });

        return Ok(next.run(req).await);
    }

    //BEARER TOKEN
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = jwt::validate(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // valida UUID
    // let user_uuid = Uuid::parse_str(&claims.sub)
    //     .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let exists: (bool,) =
        sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)"
        )
        .bind(claims.sub.clone())
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !exists.0 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    req.extensions_mut().insert(CurrentUser {
        user_id: claims.sub.clone(),
        kind: UserKind::User,
    });

    Ok(next.run(req).await)
}