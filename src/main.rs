use actix_web::{get, App, HttpServer, Responder, HttpResponse, web};
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use crate::config::{AppConfig, route_not_found};
use sqlx::mysql::MySqlPoolOptions;

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

    // create a database pool
    let db_connection = match MySqlPoolOptions::new()
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
            std::process::exit(1);
        }
    };


    let app_config = AppConfig::new(
        "Todo Backend",
        db_connection,
    );

    HttpServer::new(move ||
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(web::Data::new(AppConfig {
                name: app_config.name,
                db_pool: app_config.db_pool.clone(),
            }))
            .configure(router_config)
            .default_service(web::route().to(route_not_found))
        )
        .bind("0.0.0.0:8080")?
        .run()
        .await
}