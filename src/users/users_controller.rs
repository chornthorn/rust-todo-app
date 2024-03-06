use crate::config::AppConfig;
use crate::users::dto::{CreateUserDto, UpdateUserDto};
use crate::users::users_service::UsersService;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder, HttpMessage, HttpRequest};
use validator::Validate;
use crate::shared::token_claim::TokenClaims;

#[get("")]
async fn index(data: web::Data<AppConfig>,req: HttpRequest) -> impl Responder {
    let user_id = req.extensions().get::<TokenClaims>().unwrap().sub;
    UsersService::new(data.pool.clone()).get_all_users().await
}

#[post("")]
async fn create_user(body: web::Json<CreateUserDto>, data: web::Data<AppConfig>) -> HttpResponse {
    match body.validate() {
        Ok(_) => {
            UsersService::new(data.pool.clone())
                .create_user(body.into_inner())
                .await
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[get("{id}")]
async fn get_user_by_id(
    _: web::Data<AppConfig>,
    id: web::Path<u32>,
    data: web::Data<AppConfig>,
) -> impl Responder {
    UsersService::new(data.pool.clone())
        .get_user_by_id(id.into_inner())
        .await
}

#[patch("{id}")]
async fn update_user(
    id: web::Path<i32>,
    body: web::Json<UpdateUserDto>,
    data: web::Data<AppConfig>,
) -> impl Responder {
    UsersService::new(data.pool.clone())
        .update_user(id.into_inner(), body.into_inner())
        .await
}

#[delete("{id}")]
async fn delete_user(id: web::Path<i32>, data: web::Data<AppConfig>) -> impl Responder {
    UsersService::new(data.pool.clone())
        .delete_user(id.into_inner())
        .await
}
