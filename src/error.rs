use axum::body::{
    Bytes,
    Body,
    HttpBody
};
use axum::http::header::WWW_AUTHENTICATE;
use axum::http::{
    HeaderMap,
    HeaderValue,
    Response,
    StatusCode
};
use axum::response::IntoResponse;
use axum::Json;

use serde_json::json;

use sqlx::error::DatabaseError;

#[derive(thiserror::Error,Debug)]
pub enum Error {
    #[error("authentication required")]
    Unauthorized,

    #[error("user may not perform that action")]
    Forbidden,

    #[error("request path not found")]
    NotFound,

    #[error("password hash error")]
    HashError,

    #[error("wrong password")]
    InvalidPassword,

    #[error("error occured with the database")]
    Sqlx(#[from] sqlx::Error),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error("unprocessable request: {0}")]
    UnprocessableEntity(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        let (status, error_message) = match self {
            Error::Sqlx(_)=>{
                (StatusCode::INTERNAL_SERVER_ERROR,"Internal server error".to_string())
            },
            Error::Anyhow(_)=>{
                (StatusCode::INTERNAL_SERVER_ERROR,"Internal server error".to_string())
            },
            Error::Unauthorized=>{
                (StatusCode::UNAUTHORIZED, "Unauthorized".to_string())
            },
            Error::Forbidden => {
                (StatusCode::FORBIDDEN, "Forbidden".to_string())
            },
            Error::NotFound => {
                (StatusCode::NOT_FOUND, "NotFound".to_string())
            },
            Error::HashError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            },
            Error::InvalidPassword => {
                (StatusCode::UNAUTHORIZED, "Invalid username or password".to_string())
            },
            Error::UnprocessableEntity(msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY, msg)
            },
            _ => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error".to_string())
            }
        };

        let body = Json(json!({
            "error":{
                "error": error_message,
                "status": status.as_u16(),
            }
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
