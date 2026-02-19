use dotenvy::dotenv;
use std::env;
// use tokio::net::TcpListener;

mod config;
mod db;
mod errors;
mod handlers;
mod middleware;
mod models;
mod routes;
mod security;
mod seed;
mod services;
mod graphql;
mod state;

pub use state::AppState;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = db::mysql::create_pool(&database_url).await;

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "seed" {
        seed::seed_db(&pool).await;
        return;
    }

    let app = routes::create_router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind");

    println!("🚀 Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
