#![allow(unused)]

use crate::auth::dto::{LoginDto, RegisterDto};
use actix_web::HttpResponse;
use sqlx::MySqlPool;
use crate::auth::auth_repository::AuthRepository;
use crate::shared::response::JsonResponder;
use crate::shared::bcrypt_helper::BcryptHelper;
use crate::users::dto::CreateUserDto;
use crate::users::users_service::UsersService;

pub struct AuthService {
    pool: MySqlPool,
}

impl AuthService {
    pub fn new(pool: MySqlPool) -> Self {
        AuthService { pool }
    }

    fn repository(&self) -> AuthRepository {
        AuthRepository::new(self.pool.clone())
    }

    pub async fn login(self, login: LoginDto) -> HttpResponse {
        let repository = self.repository();
        let user = repository.get_user_by_email(&login.email).await;

        match user {
            Ok(user) => {
                if BcryptHelper::verify_hash(&login.password, &user.password) {
                    JsonResponder::ok(
                        "login successfully",
                        Some(serde_json::to_value(user).unwrap()),
                    )
                } else {
                    JsonResponder::bad_request("Invalid password")
                }
            }
            Err(e) => JsonResponder::match_err(e),
        }
    }

    pub async fn register(self, register_dto: RegisterDto) -> HttpResponse {
        let repository = self.repository();
        let user = repository.get_user_by_email(&register_dto.email).await;

        if user.is_ok() {
            return JsonResponder::bad_request("User already exists");
        }

        let users_service = UsersService::new(self.pool);
        users_service.create_user(CreateUserDto {
            username: register_dto.username,
            password: register_dto.password,
            email: register_dto.email,
        }).await
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
