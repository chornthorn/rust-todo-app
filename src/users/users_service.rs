use actix_web::{HttpResponse, Responder};
use crate::users::dto::CreateUserDto;
use crate::users::dto::UpdateUserDto;
use crate::users::entities::User;
use crate::users::users_repository::{UserRepository, UsersRepository};

pub struct UsersService {}

impl UsersService {
    pub fn new() -> Self {
        UsersService {}
    }

    fn repository(&self) -> UsersRepository {
        UsersRepository {}
    }

    pub async fn create_user(&self, dto: CreateUserDto) -> HttpResponse {
        let repository = self.repository();
        match repository.create_user(dto.username.clone(), dto.password.clone(), dto.email.clone()).await {
            Ok(user) => {
                HttpResponse::Ok().json(User {
                    id: user.id,
                    username: dto.username,
                    password: dto.password,
                    email: dto.email,
                })
            }
            Err(err) => {
                HttpResponse::InternalServerError().json(format!("Error: {}", err))
            }
        }
    }

    pub async fn get_user_by_id(&self, id: i32) -> impl Responder {
        let repository = self.repository();
        match repository.get_user_by_id(id).await {
            Ok(user) => {
                HttpResponse::Ok().json(user)
            }
            Err(err) => {
                HttpResponse::InternalServerError().json(format!("Error: {}", err))
            }
        }
    }

    pub async fn update_user(&self, id: i32, dto: UpdateUserDto) -> impl Responder {
        let repository = self.repository();
        match repository.update_user(id, dto.username, dto.password, dto.email).await {
            Ok(user) => {
                HttpResponse::Ok().json(user)
            }
            Err(err) => {
                HttpResponse::InternalServerError().json(format!("Error: {}", err))
            }
        }
    }

    pub async fn delete_user(&self, id: i32) -> impl Responder {
        let repository = self.repository();
        match repository.delete_user(id).await {
            Ok(msg) => {
                HttpResponse::Ok().json(msg)
            }
            Err(err) => {
                HttpResponse::InternalServerError().json(format!("Error: {}", err))
            }
        }
    }
}
