use crate::shared::bcrypt_helper::BcryptHelper;
use crate::shared::response::JsonResponder;
use crate::users::dto::CreateUserDto;
use crate::users::dto::UpdateUserDto;
use crate::users::users_repository::{UserRepository, UsersRepository};
use actix_web::{HttpResponse, Responder};
use sqlx::MySqlPool;

pub struct UsersService {
    pool: MySqlPool,
}

impl UsersService {
    pub fn new(pool: MySqlPool) -> Self {
        UsersService { pool }
    }

    fn repository(self) -> UsersRepository {
        UsersRepository::new(self.pool.clone())
    }

    pub async fn get_all_users(self) -> HttpResponse {
        let repository = self.repository();
        match repository.get_all_users().await {
            Ok(users) => JsonResponder::ok(
                "Retrieved users successfully",
                Some(serde_json::to_value(users).unwrap()),
            ),
            Err(err) => JsonResponder::match_err(err),
        }
    }

    pub async fn create_user(self, dto: CreateUserDto) -> HttpResponse {
        let repository = self.repository();

        // hash the password
        let hashed_password = BcryptHelper::hash_text(&dto.password);
        match hashed_password {
            Ok(hashed_password) => {
                let dto = CreateUserDto {
                    password: hashed_password,
                    ..dto // merge the hashed password with the dto
                };

                // create the user
                match repository.create_user(dto.clone()).await {
                    Ok(user) => JsonResponder::created(
                        "User created successfully",
                        Some(serde_json::to_value(user).unwrap()),
                    ),
                    Err(err) => JsonResponder::match_err(err),
                }
            }
            Err(_) => JsonResponder::bad_request("Something went wrong."),
        }
    }

    pub async fn get_user_by_id(self, id: u32) -> impl Responder {
        let repository = self.repository();
        match repository.get_user_by_id(id).await {
            Ok(user) => JsonResponder::ok(
                "Retrieved user successfully",
                Some(serde_json::to_value(user).unwrap()),
            ),
            Err(err) => JsonResponder::match_err(err),
        }
    }

    pub async fn update_user(self, id: i32, dto: UpdateUserDto) -> impl Responder {
        let repository = self.repository();
        match repository.update_user(id, dto).await {
            Ok(user) => JsonResponder::ok(
                "User updated successfully",
                Some(serde_json::to_value(user).unwrap()),
            ),
            Err(err) => JsonResponder::match_err(err),
        }
    }

    pub async fn delete_user(self, id: i32) -> impl Responder {
        let repository = self.repository();
        match repository.delete_user(id).await {
            Ok(msg) => JsonResponder::ok(
                "User deleted successfully",
                Some(serde_json::to_value(msg).unwrap()),
            ),
            Err(err) => JsonResponder::match_err(err),
        }
    }
}
