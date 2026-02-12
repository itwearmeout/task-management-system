use axum::body::{
    Bytes,
    Full,
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
use std::borrow::Cow;
use std::collections::Hashmap;

#[derive(thiserror::Error,Debug)]
pub enum Error {
    #[error("authentication required")]
    Unautherize,

    #[error("user may not perform that action")]
    Forbidden,

    #[error("request path not found")]
    NotFound,

    #[error("error in the request body")]
    UnprocessableEntry {
        errors: Hashmap<Cow<'static, str>, Vec<Cow<'static,str>>>,
    },

    #[error("error occured with the database")]
    Sqlx(#[from] sqlx::Error),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::error),
}

