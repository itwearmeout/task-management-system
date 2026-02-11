use axum::{
    {Router, Json},
    routing::{get, post},

};

pub mod task;

pub use task::{
    task_get,
    task_post,
    task_put,
};

pub fn router() -> Router {
    Router::new()
        .route("/get", get(task_get))
        .route("/post",post(task_post))
}