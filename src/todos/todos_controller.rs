use actix_web::{delete, get, post, web, Responder, patch};
use crate::todos::dto::{CreateTodoDto, UpdateTodoDto};
use crate::todos::todos_service::TodosService;


#[get("/")]
async fn get_all_todos() -> impl  Responder {
    let service = TodosService::new();
    service.get_all().await
}

#[post("/")]
async fn create_todo(body: web::Json<CreateTodoDto>) -> impl Responder {
    let service = TodosService::new();
    service.create(body.into_inner()).await
}

#[delete("/{id}")]
async fn delete_todo(id: web::Path<i32>) -> impl Responder {
    let service = TodosService::new();
    service.delete(id.into_inner()).await
}

#[patch("/{id}")]
async fn update_todo(id: web::Path<i32>, body: web::Json<UpdateTodoDto>) -> impl Responder {
    let service = TodosService::new();
    service.update(id.into_inner(), body.into_inner()).await
}
