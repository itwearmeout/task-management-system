use axum::{Extension, Json};
use uuid::Uuid;
use crate::ApiContext;
use crate::error::{Error, Result};
use crate::user::auth::AuthUserClaim;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TaskBody<T>{
    task: T,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    user_id: Uuid,
    task_subject: String,
    due_at: DateTime<Utc>,
}
#[derive(Debug, serde::Serialize)]
pub struct GetTask {
    subject: String,
    title: String,
    url: String,
    is_complete: bool,
    start_date: String,
    dead_line: String,
}
#[derive(Debug, serde::Deserialize)]
pub struct AddTask {
    user_id: Uuid,
    task_subject: String,
    due_at: DateTime<Utc>,
}

pub async fn task_get() -> Json<TaskBody<GetTask>>{
    Json(
        TaskBody{
            task: GetTask{
                subject: "Algorithm Programming".to_string(),
                title: "Podcast Week1".to_string(),
                url: "apalah".to_string(),
                is_complete: false,
                start_date: "02/02/2026".to_string(),
                dead_line: "07/02/2026".to_string(),
            }
        }
    )
}

#[axum::debug_handler]
pub async fn task_add(_claims: AuthUserClaim, ctx: Extension<ApiContext>, req: Json<TaskBody<AddTask>>) -> Result<Json<TaskBody<Task>>>{

    let _task = sqlx::query_scalar!(
    r#"
        INSERT INTO user_tasks (user_id, task_subject, due_at)
        VALUES ($1, $2, $3)
    returning task_id
    "#,
    req.task.user_id,
    req.task.task_subject,
    req.task.due_at).fetch_one(&ctx.db).await.map_err(|e| Error::Sqlx(e))?;

    Ok(Json(TaskBody { 
        task: Task { 
            user_id: req.task.user_id, 
            task_subject: req.task.task_subject.clone(),
            due_at: req.task.due_at, 
        } 
    }))

}

pub async fn task_delete(){

}