
pub struct RolesService{
    pub name: String,
}

impl RolesService {
    pub fn new() -> Self {
        Self {
            name: "RolesService".to_string(),
        }
    }
    
    pub async fn find_all() -> String {
        "find_all".to_string()
    }
}
