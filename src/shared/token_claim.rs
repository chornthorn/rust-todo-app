use actix_web::HttpMessage;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenClaims {
    pub sub: u32,
    pub iat: usize,
    pub exp: usize,
}