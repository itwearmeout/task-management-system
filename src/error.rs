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
    UnprocessableEntity {
        errors: Hashmap<Cow<'static, str>, Vec<Cow<'static,str>>>,
    },

    #[error("error occured with the database")]
    Sqlx(#[from] sqlx::Error),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::error),
}

impl Error {
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Intor<Cow<'static, str>>
    {
        let mut error_map = Hashmap::new();

        for(key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into())
        }

        Self::UnprocessableEntity { errors: error_map }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Unautherize => StatusCode::UNAUTHERIZE,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity{ .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Sqlx(_) | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

