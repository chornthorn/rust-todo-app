use crate::shared::constant::HttpError;
use crate::users::dto::{CreateUserDto, UpdateUserDto};
use crate::users::entities::User;
use sqlx::{MySql, MySqlPool, Pool};

pub trait UserRepository {
    fn new(pool: Pool<MySql>) -> Self;
    async fn get_all_users(&self) -> Result<Vec<User>, HttpError>;
    async fn create_user(&self, dto: CreateUserDto) -> Result<User, HttpError>;
    async fn get_user_by_id(&self, id: u32) -> Result<User, HttpError>;
    async fn update_user(&self, id: i32, update_user_dto: UpdateUserDto)
        -> Result<User, HttpError>;
    async fn delete_user(&self, id: i32) -> Result<String, HttpError>;
    async fn get_user_by_email(&self, email: String) -> Result<User, HttpError>;
}

pub struct UsersRepository {
    // add your database pool here
    pub pool: MySqlPool,
}

impl UserRepository for UsersRepository {
    fn new(pool: MySqlPool) -> Self {
        UsersRepository { pool }
    }

    async fn get_all_users(&self) -> Result<Vec<User>, HttpError> {
        let users = sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
            .unwrap();

        match users.len() {
            0 => Err(HttpError::NotFound("User not found")),
            _ => Ok(users),
        }
    }

    async fn create_user(&self, dto: CreateUserDto) -> Result<User, HttpError> {
        // find existing user by email
        let existing_user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", dto.email)
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        match existing_user {
            Some(_) => Err(HttpError::BadRequest("User already exists")),
            None => {
                // create new user
                let user =
                    sqlx::query("INSERT INTO users (username, password, email) VALUES (?, ?, ?)")
                        .bind(dto.username)
                        .bind(dto.password)
                        .bind(dto.email)
                        .execute(&self.pool)
                        .await
                        .unwrap();

                match user.last_insert_id() {
                    0 => Err(HttpError::BadRequest("User not created")),
                    _ => {
                        let user = sqlx::query_as!(
                            User,
                            "SELECT * FROM users WHERE id = ?",
                            user.last_insert_id()
                        )
                        .fetch_one(&self.pool)
                        .await
                        .unwrap();
                        Ok(user)
                    }
                }
            }
        }
    }

    async fn get_user_by_id(&self, id: u32) -> Result<User, HttpError> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        match user {
            Some(user) => Ok(user),
            None => Err(HttpError::NotFound("User not found")),
        }
    }

    async fn update_user(&self, id: i32, dto: UpdateUserDto) -> Result<User, HttpError> {
        let user =
            sqlx::query("UPDATE users SET username = ?, password = ?, email = ? WHERE id = ?")
                .bind(dto.username)
                .bind(dto.password)
                .bind(dto.email)
                .bind(id)
                .execute(&self.pool)
                .await
                .unwrap();

        match user.rows_affected() {
            0 => Err(HttpError::NotFound("User not found")),
            _ => {
                let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
                    .fetch_one(&self.pool)
                    .await
                    .unwrap();
                Ok(user)
            }
        }
    }

    async fn delete_user(&self, id: i32) -> Result<String, HttpError> {
        let user = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .unwrap();

        match user.rows_affected() {
            0 => Err(HttpError::NotFound("User not found")),
            _ => Ok("User deleted successfully".to_string()),
        }
    }

    async fn get_user_by_email(&self, email: String) -> Result<User, HttpError> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        match user {
            Some(user) => Ok(user),
            None => Err(HttpError::NotFound("User not found")),
        }
    }
}
