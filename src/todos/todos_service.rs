use crate::shared::response::JsonResponder;
use crate::todos::dto::{CreateTodoDto, UpdateTodoDto};
use crate::todos::todos_repository::{MysqlTodoRepository, TodoRepository};
use actix_web::HttpResponse;

pub struct TodosService {
    pool: sqlx::MySqlPool,
}

impl TodosService {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        TodosService { pool }
    }

    fn repository(&self) -> MysqlTodoRepository {
        MysqlTodoRepository::new(self.pool.clone())
    }

    pub async fn find_all(&self) -> HttpResponse {
        let repository = self.repository();
        match repository.find_all().await {
            Ok(todos) => JsonResponder::ok(
                "Retrieved todos successfully",
                Some(serde_json::to_value(todos).unwrap()),
            ),
            Err(error) => JsonResponder::match_err(error),
        }
    }

    pub async fn find_one_by_id(&self, id: u32) -> HttpResponse {
        let repository = self.repository();
        match repository.find_one_by_id(id).await {
            Ok(todo) => JsonResponder::ok(
                "Retrieved todo successfully",
                Some(serde_json::to_value(todo).unwrap()),
            ),
            Err(error) => JsonResponder::match_err(error),
        }
    }

    pub async fn create(&self, todo: CreateTodoDto) -> HttpResponse {
        let repository = self.repository();
        match repository.create(todo).await {
            Ok(todo) => JsonResponder::created(
                "Todo created successfully",
                Some(serde_json::to_value(todo).unwrap()),
            ),
            Err(error) => JsonResponder::match_err(error),
        }
    }

    pub async fn update(&self, id: u32, todo: UpdateTodoDto) -> HttpResponse {
        let repository = self.repository();
        match repository.update(id, todo).await {
            Ok(todo) => JsonResponder::ok(
                "Todo updated successfully",
                Some(serde_json::to_value(todo).unwrap()),
            ),
            Err(error) => JsonResponder::match_err(error),
        }
    }

    pub async fn delete(&self, id: u32) -> HttpResponse {
        let repository = self.repository();
        match repository.delete(id).await {
            Ok(_) => JsonResponder::ok("Todo deleted successfully", None),
            Err(error) => JsonResponder::match_err(error),
        }
    }
}
