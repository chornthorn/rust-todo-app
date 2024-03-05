use crate::auth::dto::{LoginDto, RegisterDto};
use actix_web::HttpResponse;
use sqlx::MySqlPool;
use crate::auth::auth_repository::AuthRepository;
use crate::shared::response::JsonResponder;

pub struct AuthService {
    pool: MySqlPool,
}

impl AuthService {

    pub fn new(pool: MySqlPool) -> Self {
        AuthService { pool }
    }

    fn repository(self) -> AuthRepository {
        AuthRepository::new(self.pool)
    }

    pub async fn login(self, login: LoginDto) -> HttpResponse {
        let repository = self.repository();
        let user = repository.login(login).await;
        match user {
            Ok(user) => JsonResponder::ok(
                "login successfully",
                Some(serde_json::to_value(user).unwrap())
            ),
            Err(e) => JsonResponder::match_err(e),
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
