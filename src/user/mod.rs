use axum::{
    Router,
    routing::{post, delete},
};
pub mod users;

pub mod auth;

use users::{user_login, user_create, user_delete};

pub fn router() -> Router {
    Router::new()
        .route("/login", post(user_login))
        .route("/create", post(user_create))
        .route("/delete", delete(user_delete))
}