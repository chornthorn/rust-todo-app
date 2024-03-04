use actix_web::{get, App, HttpServer, Responder, HttpResponse, web};
use crate::config::AppConfig;

mod users;
mod config;
mod shared;
mod todos;
mod auth;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json({
        let mut response = std::collections::HashMap::new();
        response.insert("message", "Welcome to the Todo Backend Api");
        response
    })
}

fn router_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(users::router)
            .configure(todos::router)
            .configure(auth::router)
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // load environment variables
    dotenv::dotenv().ok();

    // log configuration
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let app_config = AppConfig::new("Todo Backend Api".to_string());

    HttpServer::new(move ||
        App::new()
            .app_data(app_config.clone())
            .service(index)
            .configure(router_config)
        )
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
