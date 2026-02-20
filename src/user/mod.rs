use axum::{
    Router,
    routing::{get, post},

};
use sqlx::PgPool;

pub mod users;

use users::{
    user_login,
    user_create,
};

#[derive(Clone)]
pub struct ApiContext {
    pub db: PgPool,
}

pub fn router() -> Router {
    Router::new()
        .route("/login", post(user_login))
        .route("/create",post(user_create))
}