use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Debug, Deserialize, Validate)]
pub struct CreatePermissionDto {
    pub name: Option<String>,

    #[validate(length(min = 3, max = 255))]
    pub module: String,
    
    #[validate(length(min = 3, max = 255))]
    pub action: String,
    
    #[validate(range(min = 1, max = 1000))]
    pub order: usize,
}

#[derive(Serialize, Debug, Deserialize,Validate)]
pub struct UpdatePermissionDto {
    pub id: u32,
    
    #[validate(length(min = 3, max = 255))]
    pub name: Option<String>,
    
    #[validate(length(min = 3, max = 255))]
    pub module: String,
    
    #[validate(length(min = 3, max = 255))]
    pub action: String,
    
    #[validate(range(min = 1, max = 1000))]
    pub order: usize,
}