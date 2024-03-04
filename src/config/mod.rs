#[derive(Debug, Clone)]
pub struct AppConfig {
    pub name: String,
}

impl AppConfig {
    pub fn new(name: String) -> AppConfig {
        AppConfig {
            name,
        }
    }
}