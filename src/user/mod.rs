use axum::{
    Router,
    routing::{delete, post},
};
pub mod users;

pub mod auth;

use users::{user_create, user_delete, user_login};

pub fn router() -> Router {
    Router::new()
        .route("/login", post(user_login))
        .route("/create", post(user_create))
        .route("/delete", delete(user_delete))
}
