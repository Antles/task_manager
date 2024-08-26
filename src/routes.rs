use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;

use crate::db;
use crate::models::{CreateTask, Task, UpdateTask};

pub async fn list_tasks(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    let tasks = db::list_tasks(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(tasks))
}

pub async fn create_task(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let task = db::create_task(&pool, payload)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(task))
}

pub async fn delete_task(
    State(pool): State<PgPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let deleted = db::delete_task(&pool, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
    }
}

pub async fn update_task(
    State(pool): State<PgPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let task = db::update_task(&pool, id, payload)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Task not found".to_string()))?;

    Ok(Json(task))
}
