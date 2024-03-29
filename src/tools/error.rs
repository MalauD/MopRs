use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError};
use s3::error::S3Error;
use thiserror::Error;

use crate::deezer::DeezerDownloaderError;

#[derive(Error, Debug)]
pub enum MusicError {
    #[error("DatabaseError: something went wrong with mongodb")]
    DatabaseError(#[from] mongodb::error::Error),
    #[error("Something went wrong with the request")]
    ApiBackendError(#[from] reqwest::Error),
    #[error("Something went wrong with the meilisearch request")]
    SearchBackendError(#[from] meilisearch_sdk::errors::Error),
    #[error("Something went wrong with the downloader")]
    DownloaderError(#[from] DeezerDownloaderError),
    #[error("Something went wrong with the s3 client")]
    S3Error(#[from] S3Error),
}

impl ResponseError for MusicError {
    fn status_code(&self) -> StatusCode {
        match *self {
            MusicError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MusicError::ApiBackendError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MusicError::SearchBackendError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MusicError::DownloaderError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MusicError::S3Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
    #[error("AuthenticationError: something went wrong with the authentication")]
    AuthenticationError,
    #[error("DatabaseError: something went wrong with mongodb")]
    DatabaseError(#[from] mongodb::error::Error),
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::MismatchingCredential => StatusCode::UNAUTHORIZED,
            Self::AuthenticationError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).finish()
    }
}
