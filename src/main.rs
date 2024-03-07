use crate::config::{route_not_found, AppConfig};
use crate::shared::auth_middleware::Authentication;
use crate::shared::response::JsonResponder;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{error, get, web, App, HttpResponse, HttpServer, Responder};
use sqlx::mysql::MySqlPoolOptions;
use actix_cors::Cors;

mod auth;
mod config;
mod shared;
mod todos;
mod users;
mod roles;

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
            .configure(roles::router),
    );
}

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    color_eyre::install().expect("Failed to install color_eyre");

    // load environment variables
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // log configuration
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // create a database pool
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
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
        .error_handler(|err, _req| {
            error::InternalError::from_response(err, {
                JsonResponder::bad_request("Invalid payload, please check your request body")
            }).into()
        });

    HttpServer::new(move || {

        // cors configuration
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
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
        // .workers(2)
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
