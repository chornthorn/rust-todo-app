use crate::shared::constant::HttpError;
use crate::todos::dto::{CreateTodoDto, UpdateTodoDto};
use crate::todos::entities::Todo;
use sqlx::{MySql, Pool};
use crate::shared::paginated::{PaginatedRequest, PaginatedResponse};

pub trait TodoRepository {
    fn new(pool: Pool<MySql>) -> Self;
    async fn find_all(&self, pagination: Option<PaginatedRequest>) -> Result<PaginatedResponse<Todo>, HttpError>;
    async fn find_one_by_id(&self, id: u32) -> Result<Todo, HttpError>;
    async fn create(&self, todo: CreateTodoDto) -> Result<Todo, HttpError>;
    async fn update(&self, id: u32, todo: UpdateTodoDto) -> Result<Todo, HttpError>;
    async fn delete(&self, id: u32) -> Result<(), HttpError>;
}

pub struct MysqlTodoRepository {
    pool: sqlx::MySqlPool,
}

impl TodoRepository for MysqlTodoRepository {
    fn new(pool: Pool<MySql>) -> Self {
        MysqlTodoRepository { pool }
    }

    async fn find_all(&self, pagination: Option<PaginatedRequest>) -> Result<PaginatedResponse<Todo>, HttpError> {
        match pagination {
            Some(pagination) => {
                let total = sqlx::query!("SELECT COUNT(*) as total FROM todos")
                    .fetch_one(&self.pool)
                    .await
                    .unwrap()
                    .total;

                let todos = sqlx::query_as!(
                    Todo, "SELECT * FROM todos LIMIT ? OFFSET ?", 
                    pagination.limit, (pagination.page.unwrap() - 1) * pagination.limit.unwrap()
                )
                    .fetch_all(&self.pool)
                    .await
                    .unwrap();

                match todos.len() {
                    0 => Err(HttpError::NotFound("Todo not found")),
                    _ => Ok(PaginatedResponse::new(todos, total as u32, pagination.page.unwrap(), pagination.limit.unwrap())),
                }
            }
            None => {
                let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
                    .fetch_all(&self.pool)
                    .await
                    .unwrap();

                match todos.len() {
                    0 => Err(HttpError::NotFound("Todo not found")),
                    _ => Ok(PaginatedResponse::only_item(todos.clone())),
                }
            }
        }
    }

    async fn find_one_by_id(&self, id: u32) -> Result<Todo, HttpError> {
        let todo = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        match todo {
            Some(todo) => Ok(todo),
            None => Err(HttpError::NotFound("Todo not found")),
        }
    }

    async fn create(&self, todo: CreateTodoDto) -> Result<Todo, HttpError> {
        let new_todo = sqlx::query_as!(
            Todo,
            "INSERT INTO todos (title, completed) VALUES (?, ?)",
            todo.title,
            todo.completed
        )
            .execute(&self.pool)
            .await
            .unwrap();

        match new_todo.last_insert_id() {
            0 => Err(HttpError::BadRequest("Todo not created")),
            _ => {
                let todo = sqlx::query_as!(
                    Todo,
                    "SELECT * FROM todos WHERE id = ?",
                    new_todo.last_insert_id()
                )
                    .fetch_one(&self.pool)
                    .await
                    .unwrap();
                Ok(todo)
            }
        }
    }

    async fn update(&self, id: u32, todo: UpdateTodoDto) -> Result<Todo, HttpError> {
        let updated_todo = sqlx::query("UPDATE todos SET title = ?, completed = ? WHERE id = ?")
            .bind(todo.title)
            .bind(todo.completed)
            .bind(id)
            .execute(&self.pool)
            .await;

        match updated_todo {
            Ok(_) => {
                let todo = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = ?", id)
                    .fetch_one(&self.pool)
                    .await
                    .unwrap();
                Ok(todo)
            }
            Err(error) => {
                println!("{:?}", error);
                Err(HttpError::BadRequest("Todo not updated"))
            }
        }
    }

    async fn delete(&self, id: u32) -> Result<(), HttpError> {
        let deleted_todo = sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .unwrap();

        match deleted_todo.rows_affected() {
            0 => Err(HttpError::NotFound("Todo not found")),
            _ => Ok(()),
        }
    }
}
