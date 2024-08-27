use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;

use crate::auth::{create_token, AuthUser};
use crate::db;
use crate::models::{CreateTask, CreateUser, LoginUser, Task, UpdateTask, User};

pub async fn list_tasks(
    auth: AuthUser,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    let tasks = db::list_tasks(&pool, auth.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(tasks))
}

pub async fn create_task(
    auth: AuthUser,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let task = db::create_task(&pool, payload, auth.user_id)
        .await
        .map_err(|e| {
            eprintln!("Error creating task: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(Json(task))
}

pub async fn delete_task(
    auth: AuthUser,
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
    auth: AuthUser,
    State(pool): State<PgPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let current_task = db::get_task(&pool, id)
        .await
        .map_err(|e| {
            eprintln!("Error fetching task: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Task not found".to_string()))?;

    // Prepare the update payload
    let updated_task = UpdateTask {
        title: payload.title.or(Some(current_task.title)),
        completed: Some(payload.completed.unwrap_or(current_task.completed)),
    };
    let task = db::update_task(&pool, id, updated_task)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Task not found".to_string()))?;

    Ok(Json(task))
}

pub async fn register_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    match db::create_user(&pool, &payload).await {
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
    State(pool): State<PgPool>,
    Json(payload): Json<LoginUser>,
) -> Result<Json<String>, (StatusCode, String)> {
    match db::verify_user(&pool, &payload.username, &payload.password).await {
        Ok(Some(user)) => {
            let token = create_token(&user.id.to_string()).map_err(|e| {
                eprintln!("Failed to create token: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create token".to_string(),
                )
            })?;
            Ok(Json(token))
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
