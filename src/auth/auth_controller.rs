use crate::auth::auth_service::AuthService;
use crate::auth::dto::{LoginDto, RegisterDto};
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::config::AppConfig;

#[post("/login")]
async fn login(body: web::Json<LoginDto>, data: web::Data<AppConfig>) -> impl Responder {
    AuthService::new(data.pool.clone()).login(body.into_inner()).await
}

#[post("/register")]
async fn register(dto: web::Json<RegisterDto>, data: web::Data<AppConfig>) -> impl Responder {
    AuthService::new(data.pool.clone()).register(dto.into_inner()).await
}

#[post("/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().json("Logout")
}

#[post("/refresh-token")]
async fn refresh_token() -> impl Responder {
    HttpResponse::Ok().json("Refresh")
}

#[get("/user/info")]
async fn user_info() -> impl Responder {
    HttpResponse::Ok().json("User Info")
}
