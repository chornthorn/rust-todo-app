#![allow(unused)]

use crate::config::{route_not_found, AppConfig};
use crate::shared::auth_middleware::Authentication;
use crate::shared::jwt_middleware::JwtMiddleware;
use crate::shared::response::JsonResponder;
use actix_web::guard::{Guard, GuardContext};
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{
    dev, error, get, guard, web, App, HttpResponse, HttpServer, Responder, ResponseError,
};
use futures::TryFutureExt;
use sqlx::mysql::MySqlPoolOptions;
use std::error::Error;

mod auth;
mod config;
mod shared;
mod todos;
mod users;

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
            .configure(auth::router),
    );
}

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load environment variables
    dotenv::dotenv().ok();

    // log configuration
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // create a database pool
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1)
        }
    };

    let app_config = AppConfig::new("Todo Backend", pool);

    // custom json configuration
    let json_cfg = web::JsonConfig::default()
        .error_handler(|err, req| {
            error::InternalError::from_response(err, {
                JsonResponder::bad_request("Invalid payload, please check your request body")
            }).into()
        });

    HttpServer::new(move || {
        App::new()
            .wrap(Authentication)
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(web::Data::new(AppConfig {
                name: app_config.name,
                pool: app_config.pool.clone(),
            }))
            .app_data(json_cfg.clone())
            .configure(router_config)
            .default_service(web::route().to(route_not_found))
            .service(index)
    })
        .workers(2)
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
