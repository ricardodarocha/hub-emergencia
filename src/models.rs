use serde::{Deserialize, Serialize};  
use sqlx::FromRow;  
use uuid::Uuid;  
  
#[derive(Serialize, FromRow)]  
pub struct User {  
pub id: Uuid,  
pub name: String,  
pub email: String,  
#[serde(skip_serializing)]
pub password: String,
}  
  
#[derive(Deserialize)]  
pub struct CreateUser {  
pub name: String,  
pub password: String,
pub application: Option<String>,
pub email: String,  
}  
  
#[derive(Serialize)]  
pub struct PingResponse {  
pub status: String,  
}