use crate::shared::constant::HttpError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonResponder {
    message: &'static str,
    status: u16,
    data: Option<serde_json::Value>,
}

impl JsonResponder {
    pub fn new(message: &'static str, status: u16, data: Option<serde_json::Value>) -> Self {
        JsonResponder {
            message,
            status,
            data,
        }
    }

    // new_http
    pub fn new_http(
        message: &'static str,
        status: u16,
        data: Option<serde_json::Value>,
    ) -> HttpResponse {
        HttpResponse::build(StatusCode::try_from(status).unwrap())
            .json(JsonResponder::new(message, status, data))
    }

    // to response
    pub fn ok(message: &'static str, data: Option<serde_json::Value>) -> HttpResponse {
        HttpResponse::Ok().json(JsonResponder::new(message, 200, data))
    }

    // created response
    pub fn created(message: &'static str, data: Option<serde_json::Value>) -> HttpResponse {
        HttpResponse::Created().json(JsonResponder::new(message, 201, data))
    }

    // bad request
    pub fn bad_request(message: &'static str) -> HttpResponse {
        HttpResponse::BadRequest().json(JsonResponder::new(message, 400, None))
    }

    // bad request
    pub fn match_err(http_error: HttpError) -> HttpResponse {
        match http_error {
            HttpError::BadRequest(message) => {
                HttpResponse::BadRequest().json(JsonResponder::new(message, 400, None))
            }
            HttpError::Unauthorized(message) => {
                HttpResponse::Unauthorized().json(JsonResponder::new(message, 401, None))
            }
            HttpError::NotFound(message) => {
                HttpResponse::NotFound().json(JsonResponder::new(message, 404, None))
            }
            HttpError::InternalServerError(message) => {
                HttpResponse::InternalServerError().json(JsonResponder::new(message, 500, None))
            }
            HttpError::UnprocessableEntity(message) => {
                HttpResponse::UnprocessableEntity().json(JsonResponder::new(message, 422, None))
            }
        }
    }
}
