pub struct PublicRouter {
    pub routes: Vec<String>,
}

impl PublicRouter {
    pub fn new() -> Self {
        Self {
            routes: vec![
                "/".to_string(),
                "/api/auth/login".to_string(),
                "/api/auth/register".to_string(),
                "/api/auth/token/refresh".to_string(),
            ],
        }
    }

    pub fn is_public_route(&self, path: &str) -> bool {
        self.routes.contains(&path.to_string())
    }
}