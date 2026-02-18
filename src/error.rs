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

    #[error("error occured with the database")]
    Sqlx(#[from] sqlx::Error),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        let (status, error_message) = match self {
            Error::Sqlx(_)=>{
                (StatusCode::INTERNAL_SERVER_ERROR,"Internal server error")
            },
            Error::Anyhow(_)=>{
                (StatusCode::INTERNAL_SERVER_ERROR,"Internal server error")
            },
            Error::Unauthorized=>{
                (StatusCode::UNAUTHORIZED, "Unauthorized")
            },
            Error::Forbidden => {
                (StatusCode::FORBIDDEN, "Forbidden")
            },
            Error::NotFound => {
                (StatusCode::NOT_FOUND, "NotFound")
            },
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
