use actix_web::HttpResponse;
use crate::auth::dto::{LoginDto, RegisterDto};

pub struct AuthService {}

impl AuthService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn login(&self, login: LoginDto) -> HttpResponse {
        if login.email == "admin@gmail.com" && login.password == "123456" {
            HttpResponse::Ok().json("login successfully")
        } else {
            HttpResponse::Unauthorized().json("Invalid username or password")
        }
    }

    pub async fn register(&self, register_dto: RegisterDto) -> HttpResponse {
        HttpResponse::Ok().json("register successfully")
    }

    pub async fn logout(&self, email: &str) -> HttpResponse {
        if email == "thorn@gmail.com" {
            HttpResponse::Ok().json("logout successfully")
        } else {
            HttpResponse::Unauthorized().json("Invalid email")
        }
    }


    pub async fn refresh_token(&self, email: &str) -> HttpResponse {
        if email == "thorn@gmail.com" {
            HttpResponse::Ok().json("refresh token successfully")
        } else {
            HttpResponse::Unauthorized().json("Invalid username")
        }
    }

    pub async fn user_info(&self, email: &str) -> HttpResponse {
        if email == "thorn@gmail.com" {
            HttpResponse::Ok().json("user info")
        } else {
            HttpResponse::Unauthorized().json("Invalid username")
        }
    }
}