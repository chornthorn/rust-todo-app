use actix_web::HttpMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenClaims {
    pub sub: u32,
    pub iat: usize,
    pub exp: usize,
}
