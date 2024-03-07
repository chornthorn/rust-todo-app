use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use validator::Validate;
use crate::roles::dto::CreateRoleDto;
use crate::shared::response::JsonResponder;

#[get("")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().body("find_all")
}

#[get("/{id}")]
async fn find_by_id() -> impl Responder {
    HttpResponse::Ok().body("find_by_id")
}

#[post("")]
async fn create(body: web::Json<CreateRoleDto>) -> impl Responder {
    match body.validate() {
        Ok(_) => HttpResponse::Ok().body("create"),
        Err(err) => JsonResponder::validation_error(err)
    }
}

#[patch("/{id}")]
async fn update(id: web::Path<u32>,body: web::Json<CreateRoleDto>) -> impl Responder {
    match body.validate() {
        Ok(_) => HttpResponse::Ok().body("update"),
        Err(err) => JsonResponder::validation_error(err)
    }
}

#[delete("/{id}")]
async fn delete(id: web::Path<u32>) -> impl Responder {
   let id = id.into_inner();
    HttpResponse::Ok().json(id)
}