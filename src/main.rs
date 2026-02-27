use dotenvy::dotenv;  
use std::env;  
use std::net::SocketAddr;  
use tokio::{net::TcpListener, signal};  
use tracing_subscriber;
  
mod db;  
mod handlers;  
mod models;  
mod routes;  
mod middleware;

mod jwt;
mod auth;
mod password; 
  
use db::connect;  
use handlers::AppState;  
use routes::create_router; 
  
#[tokio::main]  
async fn main() {  
dotenv().ok();  
tracing_subscriber::fmt()
    .with_env_filter(
        tracing_subscriber::EnvFilter::from_default_env()
    )
    .init();
  
let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");  
let pool = connect(&database_url).await;  
  
let state = AppState { db: pool };  
let app = create_router(state);  
  
let port: u16 = env::var("PORTAUTH")  
.unwrap_or_else(|_| "3000".to_string())  
.parse()  
.expect("[PORTAUTH] port for authentication subsystem must be a number");  
  
let addr = SocketAddr::from(([127, 0, 0, 1], port));  
tracing::info!("Listening on {addr}");  
  
let listener = TcpListener::bind(&addr).await.unwrap();  
axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await.unwrap();  

    tracing::info!("Execução interrompida. OK");
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Finished with Ctrl+C - handler not installed");

    tracing::info!("Shutdown signal received");
}