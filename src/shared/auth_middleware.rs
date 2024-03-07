use crate::shared::response::JsonResponder;
use crate::shared::router::PublicRouter;
use crate::shared::token_claim::TokenClaims;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use actix_web::{http::StatusCode, HttpMessage, HttpResponse, ResponseError};
use futures::future::{err, ok, ready, Ready};
use futures::Future;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::pin::Pin;
use std::task::{Context, Poll};

impl ResponseError for JsonResponder {
    fn error_response(&self) -> HttpResponse {
        let error_json = JsonResponder::new(&self.message, 401, None);
        HttpResponse::Unauthorized().json({ error_json })
    }
}

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // check if the request is public
        let public_routes = PublicRouter::new();
        if public_routes.is_public_route(req.path()) {
            return Box::pin(self.service.call(req));
        }

        println!("Request path: {}", req.path());

        // Extract the JWT token from the request headers
        let auth_header = req.headers().get("Authorization");

        match auth_header {
            Some(header_value) => {
                // Validate the JWT token
                let token_str = header_value.to_str().unwrap_or("");
                match validate_jwt(token_str) {
                    Ok(token_claims) => {
                        // If the JWT token is valid, call the next service
                        // Add the token claims to the request extensions

                        req.extensions_mut().insert(token_claims);
                        Box::pin(self.service.call(req))
                    }
                    Err(_) => {
                        // If the JWT token is not valid, return an error
                        Box::pin(err(Error::from(JsonResponder::new(
                            "Invalid token or token expired",
                            401,
                            None,
                        ))))
                    }
                }
            }
            None => {
                // If the JWT token is not present, return an error
                Box::pin(err(Error::from(JsonResponder::new(
                    "Token not present in the request",
                    401,
                    None,
                ))))
            }
        }
    }
}

fn validate_jwt(token: &str) -> Result<TokenClaims, ()> {
    let token = token.replace("Bearer ", "");

    match decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    ) {
        Ok(claims) => Ok(claims.claims),
        Err(_) => Err(()),
    }
}
