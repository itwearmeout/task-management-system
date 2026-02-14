use axum::{
    Router,
    routing::{get, post},

};
use sqlx::PgPool;

pub mod task;

pub use task::{
    task_get,
    task_post,
    task_put,
};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/get", get(task_get))
        .route("/post",post(task_post))
}