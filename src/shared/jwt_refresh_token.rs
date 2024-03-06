use std::future::{ready, Ready};
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use crate::shared::response::JsonResponder;
use crate::shared::token_claim::TokenClaims;

#[derive(Debug, Serialize)]
pub struct JwtRefreshToken {
    pub user_id: u32,
}

impl FromRequest for JwtRefreshToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let refresh_token: String = match req.headers().get("Authorization") {
            Some(value) => value.to_str().unwrap().to_string(),
            None => {
                let error_json = JsonResponder::new(
                    "No token provided",
                    401,
                    None,
                );
                return ready(Err(ErrorUnauthorized(error_json)));
            }
        };

        let token = refresh_token.replace("Bearer ", "");

        let refresh_secret = std::env::var("REFRESH_TOKEN_SECRET")
            .expect("Refresh secret token must be set");

        let token_claims = match decode::<TokenClaims>(
            &token,
            &DecodingKey::from_secret(refresh_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(claims) => claims,
            Err(_) => {
                let error_json = JsonResponder::new(
                    "Token is invalid or expired",
                    401,
                    None,
                );
                return ready(Err(ErrorUnauthorized(error_json)));
            }
        };

        ready(Ok(JwtRefreshToken {
            user_id: token_claims.claims.sub,
        }))
    }
}

