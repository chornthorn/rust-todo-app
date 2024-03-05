use actix_web::{HttpResponse, Responder};
use crate::shared::StdResponse;

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

// handle route not found
pub async fn route_not_found() -> impl Responder {
    HttpResponse::NotFound().json({
        let mut response = StdResponse::default();
        response.message = "Route not found".to_string();
        response.error = "not found".to_string();
        response.status = 404;
        response
    })
}