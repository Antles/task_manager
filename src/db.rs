use crate::models::{CreateTask, Task, UpdateTask};
use sqlx::PgPool;

pub async fn list_tasks(pool: &PgPool) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as!(Task, "SELECT * FROM tasks ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
}

pub async fn create_task(pool: &PgPool, new_task: CreateTask) -> Result<Task, sqlx::Error> {
    sqlx::query_as!(
        Task,
        "INSERT INTO tasks (title) VALUES ($1) RETURNING *",
        new_task.title
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

pub async fn update_task(
    pool: &PgPool,
    id: i32,
    task: UpdateTask,
) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as!(Task,
  "UPDATE tasks SET title = COALESCE($1, title), completed = COALESCE($2, completed), updated_at = NOW() WHERE id = $3 RETURNING *", task.title, task.complete, id ).fetch_optional(pool).await
}
