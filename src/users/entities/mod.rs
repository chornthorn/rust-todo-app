use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize,FromRow,Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}