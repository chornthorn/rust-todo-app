use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateRoleDto {
    #[validate(length(min = 3, max = 255))]
    pub name: String,

    #[validate(length(max = 255))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRoleDto {
    #[validate(length(min = 3, max = 255))]
    pub name: String,

    #[validate(length(max = 255))]
    pub description: Option<String>,
}