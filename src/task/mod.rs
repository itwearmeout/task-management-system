use axum::{
    Router,
    routing::{get, post},

};

use crate::ApiContext;

pub mod task;

pub use task::{
    task_get,
    task_add,
    task_delete,
};

pub fn router() -> Router {
    Router::new()
        .route("/get", get(task_get))
        .route("/post",post(task_add))
}