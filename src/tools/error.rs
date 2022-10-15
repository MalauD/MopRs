use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MusicError {
    #[error("DatabaseError: something went wrong with mongodb")]
    DatabaseError(#[from] mongodb::error::Error),
    #[error("Something went wrong with the request")]
    ApiBackendError(#[from] reqwest::Error),
}

impl ResponseError for MusicError {
    fn status_code(&self) -> StatusCode {
        match *self {
            MusicError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MusicError::ApiBackendError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).finish()
    }
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("MismatchingCredential: cannot login")]
    MismatchingCredential,
    #[error("DatabaseError: something went wrong with mongodb")]
    DatabaseError(#[from] mongodb::error::Error),
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::MismatchingCredential => StatusCode::UNAUTHORIZED,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).finish()
    }
}
