use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
