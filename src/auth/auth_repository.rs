use sqlx::MySqlPool;
use crate::shared::constant::HttpError;
use crate::users::entities::User;

pub struct AuthRepository {
    pub pool: MySqlPool,
}

impl AuthRepository {
    pub fn new(pool: MySqlPool) -> Self {
        AuthRepository { pool }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, HttpError> {
        let user = sqlx::query_as!(
            User,"SELECT * FROM users WHERE email = ?",
            email
        )
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        match user {
            Some(user) => Ok(user),
            None => Err(HttpError::NotFound("User not found")),
        }
    }
}
