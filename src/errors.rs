use std::error::Error;

use actix_web::{error::ResponseError, HttpResponse};
use deadpool_postgres::PoolError;
use thiserror::Error;

use crate::responses::api_response::ApiResponse;

#[derive(Debug, Error)]
pub enum ServiceError {
    // generic error
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("IO Error: {0}")]
    IOError(std::io::Error),

    #[error("Unable to connect to the database")]
    PoolError(PoolError),
}

impl From<std::io::Error> for ServiceError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<PoolError> for ServiceError {
    fn from(error: PoolError) -> Self {
        Self::PoolError(error)
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                ApiResponse::error("Internal server error. Please try again later")
            }
            ServiceError::BadRequest(ref message) => ApiResponse::bad_request(message),
            ServiceError::IOError(ref error) => ApiResponse::error(error.description()),
            ServiceError::PoolError(ref error) => ApiResponse::error(format!(
                "Unable to connect to the database: {}",
                error.description()
            )),
        }
    }
}
