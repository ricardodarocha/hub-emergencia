use axum::Extension;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::{Json, extract::State};
use tracing::info;
use uuid::Uuid; 

use crate::db::{DbPool, internal_error};
use crate::error::AppError;
use crate::middleware::{CurrentUser, UserKind};
use crate::models::{AppStruct, CreateUser, Denuncia, PingResponse, Ponto, Solicitacao, User, Voluntario};
use crate::pagination::{PaginatedResponse, Pagination};
use crate::password::hash_password;
use crate::payloads::{NewDenuncia, NewPonto, NewSolicitacao, NewVoluntario};
use sqlx;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
}

pub async fn ping() -> Json<PingResponse> {
    Json(PingResponse {
        status: "ok".to_string(),
    })
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    //validacao minima de segurança
    if payload.senha.len() < 6 {
        return Err((StatusCode::BAD_REQUEST, "Weak password".into()));
    }

    let id = Uuid::new_v4();
    let password_hash = hash_password(&payload.senha).map_err(|_| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Password hashing failed".to_string(),
        )
    })?;

    let res = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, nome, email, app_id, senha) 
    VALUES ($1, $2, $3, $4, $5) 
    RETURNING id, nome, email, senha",
    )
    .bind(id)
    .bind(&payload.nome)
    .bind(&payload.email)
    .bind(&payload.app_id.unwrap_or_else(|| "DEMONSTRA".into()))
    .bind(&password_hash)
    .fetch_one(&state.db)
    .await;

    match res {
        Ok(user) => Ok(Json(user)),
        Err(err) => {
            // basic error mapping; improve in Part 3
            let msg = format!("DB error: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, msg))
        }
    }
}

fn ensure_app(current: &CurrentUser) -> Result<&str, AppError> {
    if current.kind != UserKind::App {
        return Err(AppError::Unauthorized);
    }
    Ok(&current.user_id)
}

pub async fn list_users(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {

    //Apenas aplicativos podem ver usuarios
    if current_user.kind != UserKind::App {
        return Err((StatusCode::UNAUTHORIZED, "".to_string()));
    }

    //aplicativos so podem ver os proprios usuarios 
    if let Ok(authenticated_add) = ensure_app(&current_user) {
        let res = sqlx::query_as::<_, User>("SELECT id, nome, email, '*****' as senha FROM users where app_id = $1")
        .bind(authenticated_add)
        .fetch_all(&state.db)
        .await;

        match res {
            Ok(users) => Ok(Json(users)),
            Err(err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("DB error: {}", err),
            )),
        }
    } else { Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("UNAUTHORIZED"),
            ))}
}

pub async fn list_apps(
    State(state): State<AppState>,
    Extension(_current_user): Extension<CurrentUser>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginatedResponse<AppStruct>>, (StatusCode, String)> {

    let (limit, offset) = pagination.limit_offset();
    info!("{}{}", limit, offset);

    let total: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM app") 
            .fetch_one(&state.db)
            .await
            .map_err(internal_error)?;

    let rows =
        sqlx::query_as::<_, AppStruct>(
            "SELECT nome,
                responsavel,
                repositorio,
                tipo,
                url,
                metadados,
                created_at FROM app ORDER BY created_at DESC LIMIT Coalesce($1, 50) OFFSET coalesce($2, 0)"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(internal_error)?;

    Ok(Json(PaginatedResponse {
        data: rows,
        total: total.0,
        page: pagination.page.unwrap_or(1),
        per_page: limit,
    }))
}

pub async fn list_pontos(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginatedResponse<Ponto>>, (StatusCode, String)> {

    let (limit, offset) = pagination.limit_offset();

    let total: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM pontos")
            .fetch_one(&state.db)
            .await
            .map_err(internal_error)?;

    let rows =
        sqlx::query_as::<_, Ponto>(
            "select pontos.nome, pontos.app_id, app.nome as origem, categoria, pontos.tipo, municipio, endereco, telefone, bairro, horarios, pontos.responsavel, pix, cnpj, itens, gps, pontos.metadados, pontos.created_at, pontos.created_by from pontos join app on app_id = app.uid  ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(internal_error)?;

    Ok(Json(PaginatedResponse {
        data: rows,
        total: total.0,
        page: pagination.page.unwrap_or(1),
        per_page: limit,
    }))
}

pub async fn list_voluntarios(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginatedResponse<Voluntario>>, (StatusCode, String)> {

    let (limit, offset) = pagination.limit_offset();

    let total: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM voluntarios")
            .fetch_one(&state.db)
            .await
            .map_err(internal_error)?;

    let rows =
        sqlx::query_as::<_, Voluntario>(
            "select voluntarios.uid, voluntarios.nome, voluntarios.app_id, app.nome as origem, categoria, municipio, telefone, mensagem, voluntarios.metadados, voluntarios.created_by, voluntarios.created_at from voluntarios join app on app_id = app.uid  ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(internal_error)?;

    Ok(Json(PaginatedResponse {
        data: rows,
        total: total.0,
        page: pagination.page.unwrap_or(1),
        per_page: limit,
    }))
}

pub async fn list_solicitacoes(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginatedResponse<Solicitacao>>, AppError> {

    let (limit, offset) = pagination.limit_offset();

    // total
    let total: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM solicitacoes")
            .fetch_one(&state.db)
            .await?;

    // dados
    let rows =
        sqlx::query_as::<_, Solicitacao>(
            r#"
            select solicitacoes.nome, solicitacoes.app_id, app.nome as origem, categoria, municipio, telefone, mensagem, solicitacoes.votos, solicitacoes.status, solicitacoes.metadados, solicitacoes.created_by, solicitacoes.created_at from solicitacoes join app on app_id = app.uid  ORDER BY created_at DESC LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?;

    Ok(Json(PaginatedResponse {
        data: rows,
        total: total.0,
        page: pagination.page.unwrap_or(1),
        per_page: limit,
    }))
}

pub async fn list_denuncias(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginatedResponse<Denuncia>>, AppError> {

    let (limit, offset) = pagination.limit_offset();

    // total
    let total: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM denuncias")
            .fetch_one(&state.db)
            .await?;

    // dados
    let rows =
        sqlx::query_as::<_, Denuncia>(
            r#"
            select denuncias.uid, denuncias.tipo, denuncias.app_id, app.nome as origem, memorando, denuncias.url, denuncias.chave_pix, denuncias.evidencias, denuncias.metadados, denuncias.created_by, denuncias.created_at from denuncias join app on app_id = app.uid  ORDER BY created_at DESC LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?;

    Ok(Json(PaginatedResponse {
        data: rows,
        total: total.0,
        page: pagination.page.unwrap_or(1),
        per_page: limit,
    }))
}

pub async fn create_ponto(
    State(state): State<AppState>,
    Extension(current): Extension<CurrentUser>,
    Json(payload): Json<NewPonto>,
) -> Result<Json<Ponto>, AppError> {

    let app_id = ensure_app(&current)?;

    let record =
        sqlx::query_as::<_, Ponto>(
            r#"
            WITH inserted AS (
            INSERT INTO pontos
            (app_id, nome, categoria, tipo, municipio, endereco,
             telefone, bairro, horarios, responsavel,
             pix, cnpj, itens, gps, metadados)
            VALUES
            ($1, $2,
             COALESCE($3, 'Ponto de Coleta'),
             COALESCE($4, 'Voluntario'),
             COALESCE($5, 'Juiz de Fora'),
             $6, $7, $8, $9, $10,
             $11, $12, $13, $14, $15)
            RETURNING * ) 
            SELECT 
                inserted.nome,
                inserted.app_id,
                app.nome AS origem,
                inserted.categoria,
                inserted.tipo,
                inserted.municipio,
                inserted.endereco,
                inserted.telefone,
                inserted.bairro,
                inserted.horarios,
                inserted.responsavel,
                inserted.pix,
                inserted.cnpj,
                inserted.itens,
                inserted.gps,
                inserted.metadados,
                inserted.created_at,
                inserted.created_by
            FROM inserted
            JOIN app ON inserted.app_id = app.uid; "#
        )
        .bind(app_id)
        .bind(&payload.nome)
        .bind(&payload.categoria)
        .bind(&payload.tipo)
        .bind(&payload.municipio)
        .bind(&payload.endereco)
        .bind(&payload.telefone)
        .bind(&payload.bairro)
        .bind(&payload.horarios)
        .bind(&payload.responsavel)
        .bind(&payload.pix)
        .bind(&payload.cnpj)
        .bind(&payload.itens)
        .bind(&payload.gps)
        .bind(&payload.metadados)
        .fetch_one(&state.db)
        .await?;

    Ok(Json(record))
}

pub async fn create_voluntario(
    State(state): State<AppState>,
    Extension(current): Extension<CurrentUser>,
    Json(payload): Json<NewVoluntario>,
) -> Result<Json<Voluntario>, AppError> {

    let app_id = ensure_app(&current)?;

    let record =
        sqlx::query_as::<_, Voluntario>(
            r#"
            WITH inserted AS (
            INSERT INTO voluntarios
            (app_id, nome, telefone, municipio, categoria, mensagem, metadados)
            VALUES
            ($1, $2, $3,
             COALESCE($4, 'Juiz de Fora'),
             $5, $6, $7)
            RETURNING * )
            select 
                inserted.uid, 
                inserted.nome, 
                inserted.app_id, 
                app.nome as origem, 
                inserted.categoria, 
                inserted.municipio, 
                inserted.telefone, 
                inserted.mensagem, 
                inserted.metadados,
                inserted.created_by, 
                inserted.created_at 
            from inserted 
            join app on inserted.app_id = app.uid
            "#
        )
        .bind(app_id)
        .bind(&payload.nome)
        .bind(&payload.telefone)
        .bind(&payload.municipio)
        .bind(&payload.categoria)
        .bind(&payload.mensagem)
        .bind(&payload.metadados)
        .fetch_one(&state.db)
        .await?;

    Ok(Json(record))
}

pub async fn create_solicitacao(
    State(state): State<AppState>,
    Extension(current): Extension<CurrentUser>,
    Json(payload): Json<NewSolicitacao>,
) -> Result<Json<Solicitacao>, AppError> {

    let app_id = ensure_app(&current)?;

    let record =
        sqlx::query_as::<_, Solicitacao>(
            r#"
            WITH inserted AS (
            INSERT INTO solicitacoes
            (app_id, nome, telefone, municipio, categoria,
             mensagem, metadados)
            VALUES
            ($1, $2, $3,
             COALESCE($4, 'Juiz de Fora'),
             $5, $6, $7)
            RETURNING *)
            select 
                inserted.uid, 
                inserted.nome, 
                inserted.app_id,
                app.nome as origem, 
                inserted.categoria, 
                inserted.status,
                inserted.votos,
                inserted.municipio, 
                inserted.telefone, 
                inserted.mensagem, 
                inserted.metadados,
                inserted.created_by, 
                inserted.created_at 
            from inserted join app on inserted.app_id = app.uid
            "#
        )
        .bind(app_id)
        .bind(&payload.nome)
        .bind(&payload.telefone)
        .bind(&payload.municipio)
        .bind(&payload.categoria)
        .bind(&payload.mensagem)
        .bind(&payload.metadados)
        .fetch_one(&state.db)
        .await?;

    Ok(Json(record))
}

pub async fn create_denuncia(
    State(state): State<AppState>,
    Extension(current): Extension<CurrentUser>,
    Json(payload): Json<NewDenuncia>,
) -> Result<Json<Denuncia>, AppError> {

    let app_id = ensure_app(&current)?;

    let record =
        sqlx::query_as::<_, Denuncia>(
            r#"
            WITH inserted AS (
            INSERT INTO denuncias
            (app_id, tipo, memorando, url, chave_pix,
             evidencias, metadados, created_by)
            VALUES
            ($1, $2, $3,
             $4,  
             $5,  
             $6,  
             $7,
             $8)
            RETURNING * ) 
            select
                inserted.uid, 
                inserted.app_id,
                inserted.tipo, 
                app.nome as origem, 
                inserted.memorando, 
                inserted.url,  
                inserted.chave_pix, 
                inserted.evidencias, 
                inserted.metadados, 
                inserted.created_by, 
                inserted.created_at 
            from inserted 
            join app on inserted.app_id = app.uid 
            "#
        )
        .bind(app_id)
        .bind(&payload.tipo)
        .bind(&payload.memorando)
        .bind(&payload.url)
        .bind(&payload.chave_pix)
        .bind(&payload.evidencias)
        .bind(&payload.metadados)
        .bind(&payload.created_by)
        .fetch_one(&state.db)
        .await?;

    Ok(Json(record))
}