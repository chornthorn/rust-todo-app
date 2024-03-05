use sqlx::MySqlPool;
use crate::auth::dto::LoginDto;
use crate::shared::bcrypt_helper::BcryptHelper;
use crate::shared::constant::HttpError;
use crate::users::entities::User;

pub struct AuthRepository {
    pub pool: MySqlPool,
}

impl AuthRepository {
    pub fn new(pool: MySqlPool) -> Self {
        AuthRepository { pool }
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, HttpError> {
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

    // login
    pub async fn login(&self, login_dto: LoginDto) -> Result<User, HttpError> {
        let user = self.get_user_by_email(&login_dto.email).await;
        let invalid_email_or_password = HttpError::BadRequest("Invalid email or password");
        match user {
            Ok(user) => {
                let is_valid = BcryptHelper::verify_hash(&login_dto.password, &user.password);
                if is_valid {
                    Ok(user)
                } else {
                    Err(invalid_email_or_password)
                }
            }
            Err(e) => Err(invalid_email_or_password),
        }
    }
}
