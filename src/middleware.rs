use axum::{
    extract::{State},
    http::{Request, StatusCode},
    response::Response,
    middleware::Next, 
};
 
// use headers::{Authorization, authorization::Bearer};
use axum::body::Body; 
use uuid::Uuid;

use crate::handlers::AppState; 
use crate::jwt;

#[allow(dead_code)]
#[derive(Clone)]
pub struct CurrentUser {
    user_id: Uuid,
}

pub async fn auth_middleware(
    State(_state): State<AppState>,
    // headers: HeaderMap,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {

    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = jwt::validate(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(CurrentUser {
        user_id: claims.sub.clone(),
    });

    Ok(next.run(req).await)
}