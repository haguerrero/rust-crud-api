use sqlx::{MySqlPool, mysql::MySqlPoolOptions};


pub async fn create_pool(database_url: &str) -> MySqlPool {
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Failed to create MySQL connection pool")
}