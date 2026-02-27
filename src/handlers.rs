use axum::{extract::State, Json}; 
use axum::http::StatusCode; 
use uuid::Uuid;  
  
use crate::db::DbPool;  
use crate::models::{CreateUser, PingResponse, User};  
use sqlx;  
use crate::password::hash_password; 
  
#[derive(Clone)]  
pub struct AppState {  
pub db: DbPool,  
}  
  
pub async fn ping() -> Json<PingResponse> {  
    Json(PingResponse {  
    status: "ok".to_string(),  
})  
}

pub async fn create_user(State(state): State<AppState>,  
	Json(payload): Json<CreateUser>,  
) -> Result<Json<User>, (StatusCode, String)> {  
    //validacao minima de segurança
    if payload.password.len() < 6 {
        return Err((StatusCode::BAD_REQUEST, "Weak password".into()));
    }

    let id = Uuid::new_v4(); 
    let password_hash = hash_password(&payload.password)
        .map_err(|_| (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Password hashing failed".to_string(),
        ))?;

    let res = sqlx::query_as::<_, User>(  
    "INSERT INTO users (id, name, email, application, password) 
    VALUES ($1, $2, $3, $4, $5) 
    RETURNING id, name, email, password",  
)  
.bind(id)  
.bind(&payload.name)  
.bind(&payload.email)  
.bind(&payload.application.unwrap_or_else(|| "AURA".into()))  
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

pub async fn list_users(  
State(state): State<AppState>,  
) -> Result<Json<Vec<User>>, (StatusCode, String)> {  
let res = sqlx::query_as::<_, User>(
"SELECT id, name, email, '*****' as password FROM users")  
.fetch_all(&state.db)  
.await;  
  
match res {  
Ok(users) => Ok(Json(users)),  
Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, 
format!("DB error: {}", err))),  
}  
}
