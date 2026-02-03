mod app_state;
mod handlers;
mod models;
mod requests;
mod router;

use axum::http::Method;
use sqlx::mysql::MySqlPoolOptions;
use tower_http::cors::{Any, CorsLayer};

use app_state::AppState;

#[tokio::main]
async fn main() {
    let database_url = "mysql://todo_user:password@localhost:3306/todo_db";

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to MySQL");

    let state = AppState { db: pool };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let app = router::create_router(state).layer(cors);

    println!("🚀 Rust TODO API running on http://localhost:8000");

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
