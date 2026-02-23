use axum::{
    Router,
    routing::{post},

};

pub mod users;

pub mod auth;

use users::{
    user_login,
    user_create,
};

use crate::ApiContext;

pub fn router() -> Router {
    Router::new()
        .route("/login", post(user_login))
        .route("/create",post(user_create))
}