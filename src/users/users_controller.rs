use actix_web::{delete, get, post, web, Responder, patch, HttpResponse};
use validator::Validate;
use crate::config::AppConfig;
use crate::users::dto::{CreateUserDto, UpdateUserDto};
use crate::users::users_service::UsersService;

#[get("/")]
async fn index() -> impl Responder {
    UsersService::new().get_user_by_id(1).await
}

#[post("/")]
async fn create_user(body: web::Json<CreateUserDto>) -> HttpResponse {
    match body.validate() {
        Ok(_) => UsersService::new().create_user(body.into_inner()).await,
        Err(err) => HttpResponse::BadRequest().json(err)
    }
}

#[get("/{id}")]
async fn get_user_by_id(_: web::Data<AppConfig>, id: web::Path<i32>) -> impl Responder {
    UsersService::new().get_user_by_id(id.into_inner()).await
}

#[patch("/{id}")]
async fn update_user(id: web::Path<i32>, body: web::Json<UpdateUserDto>) -> impl Responder {
    UsersService::new().update_user(id.into_inner(), body.into_inner()).await
}

#[delete("/{id}")]
async fn delete_user(id: web::Path<i32>) -> impl Responder {
    UsersService::new().delete_user(id.into_inner()).await
}