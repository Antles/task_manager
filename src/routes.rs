use crate::auth::{create_token, AuthUser};
use crate::db;
use crate::models::{CreateTask, CreateUser, LoginUser, Task, UpdateTask, User};
use crate::AppState;
use axum::{extract::State, http::StatusCode, Extension, Json};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;

pub async fn list_tasks(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    let tasks = db::list_tasks(&state, auth.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(tasks))
}

pub async fn create_task(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let task = db::create_task(&state, payload, auth.user_id)
        .await
        .map_err(|e| {
            eprintln!("Error creating task: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    // Broadcast the new task
    let _ = state.task_tx.send(serde_json::to_string(&task).unwrap());

    Ok(Json(task))
}

pub async fn delete_task(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let deleted = db::delete_task(&state, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if deleted {
        // Broadcast the deleted task ID
        let _ = state
            .task_tx
            .send(serde_json::to_string(&json!({ "deleted": id })).unwrap());
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
    }
}

pub async fn update_task(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let current_task = db::get_task(&state, id)
        .await
        .map_err(|e| {
            eprintln!("Error fetching task: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Task not found".to_string()))?;

    let updated_task = UpdateTask {
        title: payload.title.or(Some(current_task.title)),
        completed: Some(payload.completed.unwrap_or(current_task.completed)),
        category: payload.category.or(Some(current_task.category)),
    };
    let task = db::update_task(&state, id, updated_task)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Task not found".to_string()))?;

    // Broadcast the updated task
    let _ = state.task_tx.send(serde_json::to_string(&task).unwrap());

    Ok(Json(task))
}

pub async fn register_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    match db::create_user(&state, &payload).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            eprintln!("Failed to create user: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create user".to_string(),
            ))
        }
    }
}

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<LoginUser>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match db::verify_user(&state, &payload.username, &payload.password).await {
        Ok(Some(user)) => {
            let token = create_token(&user.id.to_string()).map_err(|e| {
                eprintln!("Failed to create token: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create token".to_string(),
                )
            })?;
            Ok(Json(json!({
                "token": token,
                "username": user.username
            })))
        }
        Ok(None) => Err((
            StatusCode::UNAUTHORIZED,
            "Invalid username or password".to_string(),
        )),
        Err(e) => {
            eprintln!("Database error during login: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ))
        }
    }
}
