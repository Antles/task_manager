use crate::models::{CreateTask, CreateUser, Task, UpdateTask, User};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;

pub async fn list_tasks(pool: &PgPool, user_id: i32) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as!(
        Task,
        "SELECT * FROM tasks WHERE user_id = $1 ORDER BY created_at DESC",
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn create_task(
    pool: &PgPool,
    new_task: CreateTask,
    user_id: i32,
) -> Result<Task, sqlx::Error> {
    sqlx::query_as!(
        Task,
        r#"
      INSERT INTO tasks (title, completed, user_id)
      VALUES ($1, $2, $3)
      RETURNING id, title, completed, created_at, updated_at, user_id
      "#,
        new_task.title,
        new_task.completed,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_task(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn get_task(pool: &PgPool, id: i32) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as!(Task, "SELECT * FROM tasks WHERE id = $1", id)
        .fetch_optional(pool)
        .await
}

pub async fn update_task(
    pool: &PgPool,
    id: i32,
    task: UpdateTask,
) -> Result<Option<Task>, sqlx::Error> {
    let result = sqlx::query_as!(Task,
  r#"UPDATE tasks SET title = COALESCE($1, title), completed = COALESCE($2, completed), updated_at = NOW() WHERE id = $3 RETURNING *"#, task.title, task.completed, id ).fetch_optional(pool).await?;

    Ok(result)
}

pub async fn create_user(pool: &PgPool, user: &CreateUser) -> Result<User, sqlx::Error> {
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
    .fetch_one(pool)
    .await
}

pub async fn get_user_by_username(
    pool: &PgPool,
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
    .fetch_optional(pool)
    .await
}

pub async fn verify_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<Option<User>, sqlx::Error> {
    if let Some(user) = get_user_by_username(pool, username).await? {
        if verify(password, &user.password_hash).unwrap_or(false) {
            Ok(Some(user))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
