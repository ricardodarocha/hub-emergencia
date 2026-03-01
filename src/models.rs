use serde::{Deserialize, Serialize};  
use sqlx::FromRow;   
use chrono::{DateTime, Utc};
use serde_json::Value; 
  
#[derive(Serialize, FromRow)]  
pub struct User {  
pub id: String,  
pub nome: String,  
pub email: String,  
#[serde(skip_serializing)]
pub senha: String,
}  
  
#[derive(Deserialize)]  
pub struct CreateUser {  
pub nome: String,  
pub senha: String,
pub app_id: Option<String>,
pub email: String,  
}  
  
#[derive(Serialize)]  
pub struct PingResponse {  
pub status: String,  
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AppStruct {
    pub nome: Option<String>,
    pub responsavel: Option<String>,
    pub repositorio: Option<String>,
    pub tipo: String,
    pub url: String,
    pub metadados: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ponto { 
    pub nome: String,
    pub origem: String,
    pub categoria: String,
    pub tipo: String,
    pub municipio: String,
    pub endereco: Option<String>,
    pub telefone: String,
    pub bairro: String,
    pub horarios: Option<String>,
    pub responsavel: Option<String>,
    #[serde(skip_serializing)]
    pub app_id: String,
    pub pix: Option<String>,
    pub cnpj: Option<String>,
    pub itens: Option<String>,
    pub gps: Option<String>,
    pub metadados: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct VotoPonto {
    #[serde(skip_serializing)]
    pub app_id: String,
    pub ponto_id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ItemPonto {
    pub item: String,
    pub aceitando: Option<bool>,
    pub recusando: Option<bool>,
    pub estoque: Option<i32>,
    #[serde(skip_serializing)]
    pub app_id: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Voluntario {
    pub uid: String,
    #[serde(skip_serializing)]
    pub app_id: String,
    pub nome: String,
    pub telefone: String,
    pub municipio: String,
    pub categoria: Option<String>,
    pub mensagem: String,
    pub metadados: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Solicitacao {  
    pub nome: String,
    pub origem: String,
    pub telefone: String,
    pub municipio: String,
    pub categoria: Option<String>,
    pub metadados: Option<Value>,
    pub mensagem: String,
    pub created_at: DateTime<Utc>,
    pub status: String,
    pub votos: Option<i32>,
    #[serde(skip_serializing)]
    pub app_id: String,
}
 
 #[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Denuncia {
    pub uid: String,
    pub tipo: String,
    pub memorando: Option<String>,
    pub url: String,
    pub chave_pix: Option<String>,
    pub evidencias: Option<String>, 
    pub metadados: Option<Value>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub app_id: String,
}