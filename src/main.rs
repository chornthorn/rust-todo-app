use actix_web::{get, App, HttpServer, Responder, web, HttpResponse};
use crate::config::AppConfig;

mod users;
mod config;
mod shared;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json({
        let mut response = std::collections::HashMap::new();
        response.insert("message", "Welcome to the Todo Backend Api");
        response
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = AppConfig::new("Todo Backend Api".to_string());

    HttpServer::new(move || App::new()
        .app_data(app_config.clone())
        .service(index)
        .configure(users::config))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
