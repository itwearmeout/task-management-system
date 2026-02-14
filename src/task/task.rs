use axum::Json;
use crate::error::{Error, Result};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TaskBody<T>{
    task: T,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Task {
    id: String,
    subject: String,
    title: String,
    url: String,
    start_date: String,
    dead_line: String,
}
#[derive(Debug, serde::Serialize)]
pub struct GetTask {
    subject: String,
    title: String,
    url: String,
    start_date: String,
    dead_line: String,

}

pub async fn task_get() -> Json<TaskBody<GetTask>>{
    Json(
        TaskBody{
            task: GetTask{
                subject: "Algorithm Programming".to_string(),
                title: "Podcast Week1".to_string(),
                url: "apalah".to_string(),
                start_date: "02/02/2026".to_string(),
                dead_line: "07/02/2026".to_string(),
            }
        }
    )
}

pub async fn task_post(){

}

pub async fn task_put(){

}