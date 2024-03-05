use actix_web::Responder;
use sqlx::{MySqlPool};
use crate::shared::response::JsonResponder;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub name: &'static str,
    pub db_pool: MySqlPool,
}

impl AppConfig {
    pub fn new(name: &'static str, db_pool: MySqlPool) -> Self {
        Self {
            name,
            db_pool,
        }
    }
}

// handle route not found
pub async fn route_not_found() -> impl Responder {
    JsonResponder::new_http("Route not found", 404, None)
}