mod db;
mod models;
mod routes;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use sqlx::PgPool;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    let app = Router::new()
        .route("/tasks", get(routes::list_tasks))
        .route("/tasks", post(routes::create_task))
        .route("/tasks/:id", delete(routes::delete_task))
        .route("/tasks/:id", patch(routes::update_task))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
