use crate::auth::dto::{LoginDto, RegisterDto};
use crate::shared::bcrypt_helper::BcryptHelper;
use crate::shared::response::JsonResponder;
use crate::shared::token_claim::TokenClaims;
use crate::users::dto::CreateUserDto;
use crate::users::users_repository::{UserRepository, UsersRepository};
use actix_web::HttpResponse;
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use sqlx::MySqlPool;
use crate::users::entities::User;

pub struct AuthService {
    pool: MySqlPool,
}

impl AuthService {
    pub fn new(pool: MySqlPool) -> Self {
        AuthService { pool }
    }

    fn user_repository(&self) -> UsersRepository {
        UsersRepository::new(self.pool.clone())
    }

    pub async fn login(self, login: LoginDto) -> HttpResponse {
        let repository = self.user_repository();
        let user = repository.get_user_by_email(login.email).await;

        match user {
            Ok(user) => {
                if BcryptHelper::verify_hash(&login.password, &user.password) {
                    let (access_token, refresh_token) = self.generate_token(user.id);
                    JsonResponder::ok(
                        "User logged in successfully",
                        Some(Self::create_response(access_token, refresh_token, user).await),
                    )
                } else {
                    JsonResponder::bad_request("Invalid password")
                }
            }
            Err(e) => JsonResponder::match_err(e),
        }
    }

    pub async fn register(self, register_dto: RegisterDto) -> HttpResponse {
        let repository = self.user_repository();
        let user = repository
            .get_user_by_email(register_dto.email.clone())
            .await;

        if user.is_ok() {
            return JsonResponder::bad_request("User already exists");
        }

        let hashed_password = BcryptHelper::hash_text(&register_dto.password);
        match hashed_password {
            Ok(hashed_password) => {
                let dto = CreateUserDto {
                    password: hashed_password,
                    username: register_dto.username,
                    email: register_dto.email,
                };

                match repository.create_user(dto.clone()).await {
                    Ok(user) => {
                        let (access_token, refresh_token) = self.generate_token(user.id);
                        JsonResponder::ok(
                            "User signed up successfully",
                            Some(Self::create_response(access_token, refresh_token, user).await),
                        )
                    }
                    Err(err) => JsonResponder::match_err(err),
                }
            }
            Err(_) => JsonResponder::bad_request("Something went wrong."),
        }
    }

    pub async fn refresh_token(self, user_id: u32) -> HttpResponse {
        // validate the user in the database
        let repository = self.user_repository();
        let user = repository.get_user_by_id(user_id).await;
        match user {
            Ok(user) => {
                let (access_token, refresh_token) = self.generate_token(user.id);
                JsonResponder::ok(
                    "Token refreshed successfully",
                    Some(Self::create_response(access_token, refresh_token, user).await),
                )
            }
            Err(e) => JsonResponder::match_err(e),
        }
    }

    pub async fn user_info(&self, user_id: u32) -> HttpResponse {
        let repository = self.user_repository();
        let user = repository.get_user_by_id(user_id).await;
        match user {
            Ok(user) => JsonResponder::ok(
                "User info",
                Some(serde_json::to_value(user).unwrap()),
            ),
            Err(e) => JsonResponder::match_err(e),
        }
    }

    // Generate access and refresh tokens
    pub fn generate_token(&self, user_id: u32) -> (String, String) {
        let access_token = self.create_token(user_id, 60, "ACCESS_TOKEN_SECRET");
        let refresh_token = self.create_token(user_id, 60 * 24 * 7, "REFRESH_TOKEN_SECRET");
        (access_token, refresh_token)
    }

    // Create a token
    fn create_token(&self, user_id: u32, minutes: i64, secret_key: &str) -> String {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + TimeDelta::try_minutes(minutes).unwrap()).timestamp() as usize;
        let claims: TokenClaims = TokenClaims { sub: user_id, exp, iat };

        let secret = std::env::var(secret_key).expect(&format!("{} must be set", secret_key));
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
            .unwrap();

        token
    }

    async fn create_response(
        access_token: String,
        refresh_token: String,
        user: User,
    ) -> serde_json::Value {

        serde_json::to_value(json!(
            {
                "token": {
                    "access_token": access_token,
                    "refresh_token": refresh_token
                },
                "user": {
                    "id": user.id,
                    "username": user.username,
                    "email": user.email,
                    "created_at": user.created_at,
                }
            }
        )).unwrap()
    }
}
