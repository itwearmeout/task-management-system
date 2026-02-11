use axum::Json;

#[derive(Debug, serde::Serialize)]
pub struct Task {
    id: String,
    subject: String,
    title: String,
    url: String,
    start_date: String,
    dead_line: String,
}

pub async fn task_get() -> Json<Task>{
    Json::from(
        Task{
            id: uuid::Uuid::new_v4().to_string(),
            subject: "Algorithm Programming".to_string(),
            title: "Podcast Week1".to_string(),
            url: "apalah".to_string(),
            start_date: "02/02/2026".to_string(),
            dead_line: "07/02/2026".to_string(),
        }
    )
}

pub async fn task_post(){

}

pub async fn task_put(){

}