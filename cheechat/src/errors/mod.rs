use actix_web::{HttpResponse, ResponseError};
use sqlx::Error as DbError;
use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum ApiError {
    NotFound,
    DbError(DbError),
    AuthError,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ApiError::NotFound => HttpResponse::NotFound().finish(),
            ApiError::DbError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            },
            ApiError::AuthError => HttpResponse::Unauthorized().finish(),
        }
    }
}
