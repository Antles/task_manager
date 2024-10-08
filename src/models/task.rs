use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub completed: bool,
    pub category: String,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub category: Option<String>,
}
