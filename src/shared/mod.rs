#![allow(unused)]

pub mod auth_middleware;
pub mod bcrypt_helper;
pub mod constant;
pub mod jwt_middleware;
pub mod jwt_refresh_token;
pub mod response;
pub mod router;
pub mod token_claim;

use crate::shared::jwt_middleware::JwtMiddleware;
use actix_web::Responder;
use regex::Regex;

pub struct Regexes {}
impl Regexes {
    pub fn new(pattern: &str) -> Regex {
        Regex::new(pattern).unwrap()
    }

    pub fn username_regex() -> Regex {
        Regexes::new(r"^[a-zA-Z0-9_]+$")
    }

    pub fn email_regex() -> Regex {
        Regexes::new(r"^[a-zA-Z0-9_]+@[a-zA-Z0-9_]+\.[a-zA-Z0-9_]+$")
    }

    pub fn id_regex() -> Regex {
        Regexes::new(r"^[0-9]+$")
    }
}

async fn with_auth<T: Responder>(f: impl FnOnce() -> T, _: JwtMiddleware) -> impl Responder {
    f()
}
