use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewApp {
    pub email: String,
    pub nome: Option<String>,
    pub responsavel: Option<String>,
    pub repositorio: Option<String>,
    pub tipo: Option<String>, // opcional para permitir default
    pub url: String,
    pub metadados: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewPonto {
    pub nome: String,
    pub categoria: Option<String>,
    pub tipo: Option<String>,
    pub municipio: Option<String>,
    pub endereco: Option<String>,
    pub telefone: String,
    pub bairro: String,
    pub horarios: Option<String>,
    pub responsavel: Option<String>,
    pub pix: Option<String>,
    pub cnpj: Option<String>,
    pub itens: Option<String>,
    pub gps: Option<String>,
    pub metadados: Option<Value>,
    pub created_by: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewVoluntario {
    pub nome: String,
    pub telefone: String,
    pub municipio: Option<String>,
    pub categoria: Option<String>,
    pub mensagem: String,
    pub metadados: Option<Value>,
    pub created_by: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewSolicitacao {
    pub nome: String,
    pub telefone: String,
    pub municipio: Option<String>,
    pub categoria: Option<String>,
    pub mensagem: String,
    pub metadados: Option<Value>,
    pub created_by: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewDenuncia {
    pub tipo: String,
    pub memorando: Option<String>,
    pub url: String,
    pub chave_pix: Option<String>,
    pub evidencias: Option<String>, 
    pub metadados: Option<Value>,
    pub created_by: Option<String>,
}
