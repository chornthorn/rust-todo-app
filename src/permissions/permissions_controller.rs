use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use validator::Validate;
use crate::config::AppConfig;
use crate::shared::response::JsonResponder;
use crate::permissions::dto::{CreatePermissionDto, UpdatePermissionDto};
use crate::shared::paginated::PaginatedRequest;

#[get("")]
pub async fn find_all(data: web::Data<AppConfig>, query: web::Query<PaginatedRequest>) -> impl Responder {
    match query.validate() {
        Ok(_) => (),
        Err(error) => return JsonResponder::validation_error(error),
    };
    
    HttpResponse::Ok().json("find_all")
}

#[get("{id}")]
pub async fn find_by_id(id: web::Path<u32>, data: web::Data<AppConfig>) -> impl Responder {
    HttpResponse::Ok().json("find_by_id")
}

#[post("")]
pub async fn create(body: web::Json<CreatePermissionDto>, data: web::Data<AppConfig>) -> impl Responder {
    match body.validate() {
        Ok(_) => HttpResponse::Ok().json("create"),
        Err(error) => JsonResponder::validation_error(error),
    }
}

#[delete("{id}")]
pub async fn delete(id: web::Path<u32>, data: web::Data<AppConfig>) -> impl Responder {
    HttpResponse::Ok().json("delete")
}

#[patch("{id}")]
pub async fn update(id: web::Path<u32>, body: web::Json<UpdatePermissionDto>, data: web::Data<AppConfig>) -> impl Responder {
    match body.validate() {
        Ok(_) => HttpResponse::Ok().json("update"),
        Err(error) => JsonResponder::validation_error(error),
    }
}