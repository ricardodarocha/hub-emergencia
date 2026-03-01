use axum::{routing::{get, post}, Router};  
use crate::handlers::{ping, AppState};
use crate::handlers::{list_users, list_apps, list_pontos, list_solicitacoes, list_voluntarios, list_denuncias};
use crate::handlers::{create_user, create_ponto, create_solicitacao, create_voluntario, create_denuncia};
use crate::auth::{generate_token};  
use axum::middleware::from_fn_with_state;
use crate::middleware;
  
pub fn create_router(state: AppState) -> Router {  
    let public_routes = Router::new()
        .route("/ping", get(ping))
        .route("/apps/view", get(list_apps) )
        .route("/register", post(create_user))
        .route("/login", post(generate_token).get(generate_token));

    // 🔐 Rotas privadas
    let private_routes = Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/apps", get(list_apps) )
        .route("/pontos", get(list_pontos).post(create_ponto))
        .route("/solicitacoes", get(list_solicitacoes).post(create_solicitacao))
        .route("/voluntarios", get(list_voluntarios).post(create_voluntario))
        .route("/denuncias", get(list_denuncias).post(create_denuncia))
        .route_layer(
            from_fn_with_state(state.clone(), middleware::auth_middleware)
        );
  
  Router::new()
        .merge(public_routes)
        .merge(private_routes)
        .with_state(state)
}