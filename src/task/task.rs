use axum::{Extension, Json, http::StatusCode};
use uuid::Uuid;
use crate::ApiContext;
use crate::error::{Error, Result};
use crate::user::auth::AuthUserClaim;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TaskBody<T>{
    task: T,
}

#[derive(Debug, serde::Serialize)]
pub struct MultTaskBody<T>{
    task: Vec<T>
}

#[derive(Debug, serde::Serialize)]
pub struct TaskItems {
    pub task_subject: String,
    pub due_at: DateTime<Utc>,
    pub is_complete: Option<bool>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    user_id: Uuid,
    task_subject: String,
    due_at: DateTime<Utc>,
}
#[derive(Debug, serde::Serialize)]
pub struct GetTask {
    user_id: Uuid,
    task_subject: String,
    due_at: DateTime<Utc>,
    is_complete: bool,
}
#[derive(Debug, serde::Deserialize)]
pub struct AddTask {
    user_id: Uuid,
    task_subject: String,
    due_at: DateTime<Utc>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DeleteTask {
    task_id: Uuid,
}

pub async fn task_get(claims: AuthUserClaim, ctx: Extension<ApiContext>) -> Result<Json<MultTaskBody<TaskItems>>>{

    let tasks = sqlx::query_as!(
        TaskItems,
        r#"
            SELECT task_subject, due_at, is_complete 
            FROM user_tasks 
            WHERE user_id = $1
            ORDER BY due_at
        "#,
        claims.user_id,
    ).fetch_all(&ctx.db).await.map_err(|e| Error::Sqlx(e))?;

    Ok(Json(MultTaskBody {task: tasks}))

}

pub async fn task_add(claims: AuthUserClaim, ctx: Extension<ApiContext>, req: Json<TaskBody<AddTask>>) -> Result<Json<TaskBody<Task>>>{

    let _task = sqlx::query_scalar!(
        r#"
            INSERT INTO user_tasks (user_id, task_subject, due_at)
            VALUES ($1, $2, $3)
        returning task_id
        "#,
        claims.user_id,
        req.task.task_subject,
        req.task.due_at
    ).fetch_one(&ctx.db).await.map_err(|e| Error::Sqlx(e))?;

    Ok(Json(TaskBody { 
        task: Task { 
            user_id: req.task.user_id, 
            task_subject: req.task.task_subject.clone(),
            due_at: req.task.due_at, 
        } 
    }))

}

pub async fn task_delete(
    claims: AuthUserClaim,
    ctx: Extension<ApiContext>,
    req: Json<TaskBody<DeleteTask>>,
) -> Result<StatusCode> {
    let result = sqlx::query!(
        r#"
            DELETE FROM user_tasks
            WHERE user_id = $1
              AND task_id = $2
        "#,
        claims.user_id,
        req.task.task_id,
    )
    .execute(&ctx.db)
    .await
    .map_err(Error::Sqlx)?;

    let _ = result.rows_affected();

    Ok(StatusCode::NO_CONTENT)
}