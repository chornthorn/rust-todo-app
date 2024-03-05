use crate::shared::Regexes;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    // username must be between 3 and 255 characters and not contain special characters or spaces
    static ref USERNAME_REGEX: Regex = Regexes::username_regex();

    // email must be a valid email
    static ref EMAIL_REGEX: Regex = Regexes::email_regex();
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateUserDto {
    // #[validate(
    //     length(
    //         min = 3,
    //         max = 255,
    //         message = "Username must be between 3 and 255 characters"
    //     ),
    //     regex(
    //         path = "USERNAME_REGEX",
    //         message = "Username must not contain special characters or spaces"
    //     )
    // )]
    pub username: String,

    #[validate(length(
        min = 6,
        max = 255,
        message = "Password must be between 6 and 255 characters"
    ))]
    pub password: String,

    // #[validate(regex(path = *"EMAIL_REGEX", message = "Email must be a valid email"))]
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
}
