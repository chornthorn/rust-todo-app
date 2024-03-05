use crate::todos::dto::{CreateTodoDto, UpdateTodoDto};
use crate::todos::entities::Todo;

pub trait TodoRepository {
    async fn get_all(&self) -> Result<Vec<Todo>, String>;
    async fn get_by_id(&self, id: i32) -> Result<Todo, String>;
    async fn create(&self, todo: CreateTodoDto) -> Result<Todo, String>;
    async fn update(&self, id: i32, todo: UpdateTodoDto) -> Result<Todo, String>;
    async fn delete(&self, id: i32) -> Result<(), String>;
}

pub struct InMemoryTodoRepository {}

impl TodoRepository for InMemoryTodoRepository {
    async fn get_all(&self) -> Result<Vec<Todo>, String> {
        let todos = vec![
            Todo {
                id: 1,
                title: "Todo 1".to_string(),
                completed: false,
            },
            Todo {
                id: 2,
                title: "Todo 2".to_string(),
                completed: true,
            },
        ];

        Ok(todos)
    }

    async fn get_by_id(&self, id: i32) -> Result<Todo, String> {
        Ok(Todo {
            id,
            title: "Todo 1".to_string(),
            completed: false,
        })
    }

    async fn create(&self, todo: CreateTodoDto) -> Result<Todo, String> {
        Ok(Todo {
            id: 3,
            title: todo.title,
            completed: false,
        })
    }

    async fn update(&self, id: i32, todo: UpdateTodoDto) -> Result<Todo, String> {
        Ok(Todo {
            id,
            title: if let Some(title) = todo.title {
                title
            } else {
                "Todo 1".to_string()
            },
            completed: if let Some(completed) = todo.completed {
                completed
            } else {
                false
            },
        })
    }

    async fn delete(&self, id: i32) -> Result<(), String> {
        Ok(())
    }
}
