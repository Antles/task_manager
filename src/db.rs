use crate::models::{CreateTask, CreateUser, Task, UpdateTask, User};
use crate::AppState;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::sync::Arc;

pub async fn list_tasks(state: &Arc<AppState>, user_id: i32) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as!(
        Task,
        r#"
        SELECT *
        FROM tasks
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(&state.pool)
    .await
}

pub async fn create_task(
    state: &Arc<AppState>,
    new_task: CreateTask,
    user_id: i32,
) -> Result<Task, sqlx::Error> {
    sqlx::query_as!(
        Task,
        r#"
    INSERT INTO tasks (title, completed, category, user_id)
    VALUES ($1, $2, $3, $4)
    RETURNING *
        "#,
        new_task.title,
        new_task.completed,
        new_task.category,
        user_id
    )
    .fetch_one(&state.pool)
    .await
}

pub async fn delete_task(state: &Arc<AppState>, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
        .execute(&state.pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn get_task(state: &Arc<AppState>, id: i32) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as!(
        Task,
        r#"
        SELECT *
        FROM tasks
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await
}

pub async fn update_task(
    state: &Arc<AppState>,
    id: i32,
    task: UpdateTask,
) -> Result<Option<Task>, sqlx::Error> {
    let result = sqlx::query_as!(Task,
      r#"UPDATE tasks SET title = COALESCE($1, title), completed = COALESCE($2, completed), category = $3, updated_at = NOW() WHERE id = $4 RETURNING *"#, task.title, task.completed, task.category, id ).fetch_optional(&state.pool).await?;

    Ok(result)
}

pub async fn create_user(state: &Arc<AppState>, user: &CreateUser) -> Result<User, sqlx::Error> {
    let password_hash = hash(&user.password, DEFAULT_COST).unwrap();

    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        RETURNING id, username, password_hash
        "#,
        user.username,
        password_hash
    )
    .fetch_one(&state.pool)
    .await
}

pub async fn get_user_by_username(
    state: &Arc<AppState>,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(&state.pool)
    .await
}

pub async fn verify_user(
    state: &Arc<AppState>,
    username: &str,
    password: &str,
) -> Result<Option<User>, sqlx::Error> {
    if let Some(user) = get_user_by_username(state, username).await? {
        if verify(password, &user.password_hash).unwrap_or(false) {
            Ok(Some(user))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
