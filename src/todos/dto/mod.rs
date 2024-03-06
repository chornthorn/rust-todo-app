use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone,Validate)]
pub struct CreateTodoDto {

    #[validate(length(min = 1, max = 255))]
    pub title: String,

    #[validate(range(min = 0, max = 1, message = "completed must be 0 or 1"))]
    pub completed: i8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTodoDto {
    pub title: Option<String>,
    pub completed: Option<i8>,
}
