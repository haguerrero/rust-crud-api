
// // #[tokio::main]
// // async fn main() {
// //     let app = routes::create_router();

// //     let listener = TcpListener::bind("0.0.0.0:3000")
// //         .await
// //         .expect("Failed to bind address");

// //     println!("Server running on http://0.0.0.0:3000");

// //     axum::serve(listener, app)
// //         .await
// //         .expect("Server crashed");
// // }
// // async fn main() {
// //     dotenvy::dotenv().ok();
    
// //     let database_url = std::env::var("DATABASE_URL")
// //         .expect("DATABASE_URL not set");
    
// //     let pool = MySqlPool::connect(&database_url)
// //         .await
// //         .expect("Failed to connect to MySQL database");
    
// //     let app = Router::new()
// //         .merge(routes::user_routes(pool));
    
// //     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
// //         .serve(app.into_make_service())
// //         .await
// //         .unwrap();
// // }


// // fn main() {
// //     dotenvy::dotenv().ok();
// //     tokio::runtime::Builder::new_multi_thread()
// //         .enable_all()
// //         .build()
// //         .unwrap()
// //         .block_on(app::run());
// // }


// use tokio::net::TcpListener;

// mod config;
// mod db;
// mod handlers;
// mod models;
// mod routes;

// #[tokio::main]
// async fn main() {
//     let app = routes::create_router();

//     let listener = TcpListener::bind("0.0.0.0:3000")
//         .await
//         .expect("Failed to bind");

//     println!("🚀 Server running on http://0.0.0.0:3000");

//     axum::serve(listener, app)
//         .await
//         .unwrap();
// }

use dotenvy::dotenv;
use std::env;
// use tokio::net::TcpListener;

mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod seed;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");

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

    axum::serve(listener, app)
        .await
        .unwrap();
}

