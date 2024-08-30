pub mod auth;
mod db;
mod models;
mod routes;
mod websocket;
use crate::websocket::handle_socket;
use axum::headers::HeaderValue;
use axum::http::{header, Method};
use axum::{
    extract::ws::WebSocketUpgrade,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Extension, Router,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
    task_tx: broadcast::Sender<String>,
}

pub async fn create_app() -> Router {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let frontend_url = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    let (task_tx, _) = broadcast::channel(100);

    let state = Arc::new(AppState { pool, task_tx });

    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(vec![
            header::ACCEPT,
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
        ])
        .allow_credentials(true);

    Router::new()
        .route("/register", post(routes::register_user))
        .route("/login", post(routes::login))
        .route("/tasks", get(routes::list_tasks))
        .route("/tasks", post(routes::create_task))
        .route("/tasks/:id", delete(routes::delete_task))
        .route("/tasks/:id", patch(routes::update_task))
        .route("/ws", get(ws_handler))
        .route("/health_check", get(health_check))
        .layer(cors)
        .layer(Extension(state.clone()))
        .with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn health_check() -> impl IntoResponse {
    "Success\n"
}

#[tokio::main]
async fn main() {
    let app = create_app().await;
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
