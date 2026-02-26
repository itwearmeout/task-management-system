use axum::{
    Router,
    routing::{delete, get, post},
};

pub mod task_handler;

pub use task_handler::{task_add, task_delete, task_get};

pub fn router() -> Router {
    Router::new()
        .route("/get", get(task_get))
        .route("/post", post(task_add))
        .route("/delete", delete(task_delete))
}
