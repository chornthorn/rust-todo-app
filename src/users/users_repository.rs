use crate::users::entities::User;

pub trait UserRepository {
    async fn create_user(&self, username: String, password: String, email: String) -> Result<User, String>;
    async fn get_user_by_id(&self, id: i32) -> Result<User, String>;
    async fn update_user(&self, id: i32, username: Option<String>, password: Option<String>, email: Option<String>) -> Result<User, String>;
    async fn delete_user(&self, id: i32) -> Result<String, String>;
}

pub struct UsersRepository {}

impl UserRepository for UsersRepository {
    async fn create_user(&self, username: String, password: String, email: String) -> Result<User, String> {
        Ok(User {
            id: 1,
            username,
            password,
            email,
        })
    }

    async fn get_user_by_id(&self, id: i32) -> Result<User, String> {
        let user = User {
            id,
            username: "Jonh Deo 2".to_string(),
            password: "123456".to_string(),
            email: "jonhdeo@gmail.com".to_string(),
        };
        Ok(user)
    }

    async fn update_user(&self, id: i32, username: Option<String>, password: Option<String>, email: Option<String>) -> Result<User, String> {
        let user = User {
            id,
            username: username.unwrap_or("user".to_string()),
            password: password.unwrap_or("password".to_string()),
            email: email.unwrap_or("email".to_string()),
        };
        Ok(user)
    }

    async fn delete_user(&self, id: i32) -> Result<String, String> {
        Ok(format!("User with id {} has been deleted", id))
    }
}
