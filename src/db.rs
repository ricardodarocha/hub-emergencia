use sqlx::{Pool, Postgres};  
  
pub type DbPool = Pool<Postgres>;  
  
pub async fn connect(database_url: &str) -> DbPool {  
Pool::<Postgres>::connect(database_url)  
.await  
.expect("Failed to connect to Postgres")  
}