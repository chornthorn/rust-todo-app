use crate::auth::dto::{LoginDto, RegisterDto};
use actix_web::HttpResponse;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use sqlx::MySqlPool;
use crate::shared::response::JsonResponder;
use crate::shared::bcrypt_helper::BcryptHelper;
use crate::shared::token_claim::TokenClaims;
use crate::users::dto::CreateUserDto;
use crate::users::users_repository::{UserRepository, UsersRepository};
use crate::users::users_service::UsersService;

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
                    let (access_token, refresh_token) = self.generate_token(user.id).await;
                    JsonResponder::ok(
                        "User logged in successfully",
                        Some(serde_json::to_value(
                            json!(
                                {
                                    "token": {
                                        "access_token": access_token,
                                        "refresh_token": refresh_token
                                    },
                                    "user": user,
                                }
                            )
                        ).unwrap()),
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
        let user = repository.get_user_by_email(register_dto.email.clone()).await;

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
                        let (access_token, refresh_token) = self.generate_token(user.id).await;
                        JsonResponder::ok(
                            "User signed up successfully",
                            Some(serde_json::to_value(
                                json!({
                                    "token": {
                                        "access_token": access_token,
                                        "refresh_token": refresh_token
                                    },
                                    "user": user,
                                })
                            ).unwrap()),
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
                let (access_token, refresh_token) = self.generate_token(user.id).await;
                JsonResponder::ok(
                    "Token refreshed successfully",
                    Some(serde_json::to_value(
                        json!(
                            {
                                "token": {
                                    "access_token": access_token,
                                    "refresh_token": refresh_token
                                },
                                "user": user,
                            }
                        )
                    ).unwrap()),
                )
            }
            Err(e) => JsonResponder::match_err(e),
        }
    }

    pub async fn user_info(&self, user_id: u32) -> HttpResponse {
        let repository = self.user_repository();
        let user = repository.get_user_by_id(user_id).await;
        match user {
            Ok(user) => {
                JsonResponder::ok("User info", Some(serde_json::to_value(user).unwrap()))
            }
            Err(e) => JsonResponder::match_err(e),
        }
    }

    // generate either a refresh token or an access token and return as string tuple token string
    pub async fn generate_token(&self, user_id: u32) -> (String, String) {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::minutes(60)).timestamp() as usize;
        let claims: TokenClaims = TokenClaims {
            sub: user_id,
            exp,
            iat,
        };

        let access_secret = std::env::var("ACCESS_TOKEN_SECRET")
            .expect("ACCESS_TOKEN_SECRET must be set");
        let access_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(access_secret.as_ref()),
        )
            .unwrap();

        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::days(7)).timestamp() as usize;
        let claims: TokenClaims = TokenClaims {
            sub: user_id,
            exp,
            iat,
        };

        let refresh_secret = std::env::var("REFRESH_TOKEN_SECRET")
            .expect("REFRESH_TOKEN_SECRET must be set");
        let refresh_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(refresh_secret.as_ref()),
        )
            .unwrap();

        (access_token, refresh_token)
    }
}
