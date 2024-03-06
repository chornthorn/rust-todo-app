use crate::config::AppConfig;
use crate::shared::response::JsonResponder;
use crate::todos::dto::{CreateTodoDto, UpdateTodoDto};
use crate::todos::todos_service::TodosService;
use actix_web::{delete, get, patch, post, web, Responder};
use validator::{Validate, ValidationErrors};

#[get("")]
async fn get_all_todos(data: web::Data<AppConfig>) -> impl Responder {
    let service = TodosService::new(data.pool.clone());
    service.find_all().await
}

#[get("{id}")]
async fn get_todo_by_id(id: web::Path<u32>, data: web::Data<AppConfig>) -> impl Responder {
    let service = TodosService::new(data.pool.clone());
    service.find_one_by_id(id.into_inner()).await
}

#[post("")]
async fn create_todo(body: web::Json<CreateTodoDto>, data: web::Data<AppConfig>) -> impl Responder {
    match body.validate() {
        Ok(_) => {
            let service = TodosService::new(data.pool.clone());
            service.create(body.into_inner()).await
        }
        Err(error) => JsonResponder::validation_error(error),
    }
}

#[delete("{id}")]
async fn delete_todo(id: web::Path<u32>, data: web::Data<AppConfig>) -> impl Responder {
    let service = TodosService::new(data.pool.clone());
    service.delete(id.into_inner()).await
}

#[patch("{id}")]
async fn update_todo(
    id: web::Path<u32>,
    body: web::Json<UpdateTodoDto>,
    data: web::Data<AppConfig>,
) -> impl Responder {
    let service = TodosService::new(data.pool.clone());
    service.update(id.into_inner(), body.into_inner()).await
}
