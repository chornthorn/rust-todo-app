use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: i8,
    pub created_at: Option<chrono::DateTime<Utc>>,
}
