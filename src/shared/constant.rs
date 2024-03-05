use serde::{Deserialize, Serialize};

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum HttpError {
    NotFound(&'static str),
    InternalServerError(&'static str),
    BadRequest(&'static str),
    Unauthorized(&'static str),
    UnprocessableEntity(&'static str),
}

impl From<sqlx::Error> for HttpError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => HttpError::NotFound("not_found"),
            _ => HttpError::InternalServerError("internal_server_error"),
        }
    }
}

// implement message for HttpError
impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpError::NotFound(message) => write!(f, "Not found: {}", message),
            HttpError::InternalServerError(message) => {
                write!(f, "Internal server error: {}", message)
            }
            HttpError::BadRequest(message) => write!(f, "Bad request: {}", message),
            HttpError::Unauthorized(message) => write!(f, "Unauthorized: {}", message),
            HttpError::UnprocessableEntity(message) => {
                write!(f, "Unprocessable entity: {}", message)
            }
        }
    }
}
