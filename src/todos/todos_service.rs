use actix_web::{HttpResponse, Responder};
use crate::todos::todos_repository::{InMemoryTodoRepository, TodoRepository};

pub struct TodosService {}

impl TodosService {
    pub fn new() -> Self {
        Self {}
    }

    fn repository(&self) -> InMemoryTodoRepository {
        InMemoryTodoRepository {}
    }

    pub async fn get_all(&self) -> HttpResponse {
        let repository = self.repository();
        match repository.get_all().await {
            Ok(todos) => HttpResponse::Ok().json(todos),
            Err(_) => HttpResponse::InternalServerError().finish()
        }
    }

    pub async fn get_by_id(&self, id: i32) -> HttpResponse {
        let repository = self.repository();
        match repository.get_by_id(id).await {
            Ok(todo) => HttpResponse::Ok().json(todo),
            Err(_) => HttpResponse::InternalServerError().finish()
        }
    }

    pub async fn create(&self, todo: crate::todos::dto::CreateTodoDto) -> HttpResponse {
        let repository = self.repository();
        match repository.create(todo).await {
            Ok(todo) => HttpResponse::Ok().json(todo),
            Err(_) => HttpResponse::InternalServerError().finish()
        }
    }

    pub async fn update(&self, id: i32, todo: crate::todos::dto::UpdateTodoDto) -> HttpResponse {
        let repository = self.repository();
        match repository.update(id, todo).await {
            Ok(todo) => HttpResponse::Ok().json(todo),
            Err(_) => HttpResponse::InternalServerError().finish()
        }
    }

    pub async fn delete(&self, id: i32) ->HttpResponse {
        let repository = self.repository();
        match repository.delete(id).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish()
        }
    }
}