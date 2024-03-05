use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTodoDto {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTodoDto {
    pub title: Option<String>,
    pub completed: Option<bool>,
}
