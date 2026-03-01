use axum::http::StatusCode;
use sqlx::{Pool, Postgres};  
  
pub type DbPool = Pool<Postgres>;  
  
pub async fn connect(database_url: &str) -> DbPool {  
Pool::<Postgres>::connect(database_url)  
.await  
.expect("Failed to connect to Postgres")  
}

pub fn internal_error<E: std::fmt::Display>(err: E) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("DB error: {err}"),
    )
}