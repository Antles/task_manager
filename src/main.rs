pub mod auth;
mod db;
mod models;
mod routes;
use axum::{
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

pub async fn create_app() -> Router {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    Router::new()
        .route("/register", post(routes::register_user))
        .route("/login", post(routes::login))
        .route("/tasks", get(routes::list_tasks))
        .route("/tasks", post(routes::create_task))
        .route("/tasks/:id", delete(routes::delete_task))
        .route("/tasks/:id", patch(routes::update_task))
        .layer(cors)
        .with_state(pool)
}

#[tokio::main]
async fn main() {
    let app = create_app().await;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
