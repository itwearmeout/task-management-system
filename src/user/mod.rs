use axum::{
    Router,
    routing::{post},

};

use crate::ApiContext;

pub mod users;

pub mod auth;

use users::{
    user_login,
    user_create,
};

pub fn router() -> Router {
    Router::new()
        .route("/login", post(user_login))
        .route("/create",post(user_create))
}