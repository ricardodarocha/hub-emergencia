use axum::{routing::{get, post}, Router};  
use crate::handlers::{create_user, ping, list_users, AppState};
use crate::auth::{generate_token};  
use axum::middleware::from_fn_with_state;
use crate::middleware;
  
pub fn create_router(state: AppState) -> Router {  
    let public_routes = Router::new()
        .route("/ping", get(ping))
        .route("/register", post(create_user))
        .route("/login", post(generate_token).get(generate_token));

    // 🔐 Rotas privadas
    let private_routes = Router::new()
        .route("/users", get(list_users))
        .route_layer(
            from_fn_with_state(state.clone(), middleware::auth_middleware)
        );
  
  Router::new()
        .merge(public_routes)
        .merge(private_routes)
        .with_state(state)
}