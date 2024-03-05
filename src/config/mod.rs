use crate::shared::response::JsonResponder;
use actix_web::Responder;
use sqlx::MySqlPool;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub name: &'static str,
    pub pool: MySqlPool,
}

impl AppConfig {
    pub fn new(name: &'static str, pool: MySqlPool) -> Self {
        Self { name, pool }
    }
}

// handle route not found
pub async fn route_not_found() -> impl Responder {
    JsonResponder::new_http("Route not found", 404, None)
}
