use crate::shared::response::JsonResponder;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use std::future::{ready, Ready};
use crate::shared::constant::TokenClaims;

pub struct JwtMiddleware {
    pub user_id: u32,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let token = match req.headers().get("Authorization") {
            Some(value) => value.to_str().unwrap().to_string(),
            None => {
                let error_json = JsonResponder::new("No token provided", 401, None);
                return ready(Err(ErrorUnauthorized(error_json)));
            }
        };

        let token = token.replace("Bearer ", "");

        let token_claims = match decode::<TokenClaims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        ) {
            Ok(claims) => claims,
            Err(_) => {
                let error_json = JsonResponder::new("Token is invalid or expired", 401, None);
                return ready(Err(ErrorUnauthorized(error_json)));
            }
        };

        ready(Ok(JwtMiddleware {
            user_id: token_claims.claims.sub,
        }))
    }
}
