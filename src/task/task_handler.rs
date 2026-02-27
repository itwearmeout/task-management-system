use crate::ApiContext;
use crate::error::{Error, Result};
use crate::user::auth::AuthUserClaim;
use axum::{
    Json,
    extract::{Extension, Query},
    http::StatusCode,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TaskBody<T> {
    task: T,
}

#[derive(Debug, serde::Serialize)]
pub struct MultTaskBody<T> {
    task: Vec<T>,
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

#[derive(Debug, serde::Deserialize)]
pub struct TaskSearchQuery {
    pub keyword: Option<String>,
    pub status: Option<String>,
}

fn parse_status_filter(status: Option<String>) -> Result<Option<bool>> {
    if let Some(raw) = status {
        let normalized = raw.trim().to_lowercase();

        match normalized.as_str() {
            "complete" | "completed" | "true" => Ok(Some(true)),
            "incomplete" | "pending" | "false" => Ok(Some(false)),
            _ => Err(Error::UnprocessableEntity(
                "status filter must be one of: complete, incomplete, true, false".into(),
            )),
        }
    } else {
        Ok(None)
    }
}

pub async fn task_get(
    claims: AuthUserClaim,
    ctx: Extension<ApiContext>,
) -> Result<Json<MultTaskBody<TaskItems>>> {
    let tasks = sqlx::query_as!(
        TaskItems,
        r#"
            SELECT task_subject, due_at, is_complete 
            FROM user_tasks 
            WHERE user_id = $1
            ORDER BY due_at
        "#,
        claims.user_id,
    )
    .fetch_all(&ctx.db)
    .await
    .map_err(Error::Sqlx)?;

    Ok(Json(MultTaskBody { task: tasks }))
}

pub async fn search_tasks(
    Extension(ctx): Extension<ApiContext>,
    auth_user: AuthUserClaim,
    Query(query): Query<TaskSearchQuery>,
) -> Result<Json<MultTaskBody<TaskItems>>> {
    let keyword_pattern = query.keyword.map(|raw| format!("%{}%", raw));
    let status_filter = parse_status_filter(query.status)?;

    let tasks = sqlx::query_as!(
        TaskItems,
        r#"
            SELECT task_subject, due_at, is_complete
            FROM user_tasks
            WHERE user_id = $1
              AND ($2::text IS NULL OR task_subject ILIKE $2)
              AND ($3::boolean IS NULL OR is_complete = $3)
            ORDER BY due_at
        "#,
        auth_user.user_id,
        keyword_pattern,
        status_filter,
    )
    .fetch_all(&ctx.db)
    .await
    .map_err(Error::Sqlx)?;

    Ok(Json(MultTaskBody { task: tasks }))
}

pub async fn task_add(
    claims: AuthUserClaim,
    ctx: Extension<ApiContext>,
    req: Json<TaskBody<AddTask>>,
) -> Result<Json<TaskBody<Task>>> {
    let _task = sqlx::query_scalar!(
        r#"
            INSERT INTO user_tasks (user_id, task_subject, due_at)
            VALUES ($1, $2, $3)
        returning task_id
        "#,
        claims.user_id,
        req.task.task_subject,
        req.task.due_at
    )
    .fetch_one(&ctx.db)
    .await
    .map_err(Error::Sqlx)?;

    Ok(Json(TaskBody {
        task: Task {
            user_id: req.task.user_id,
            task_subject: req.task.task_subject.clone(),
            due_at: req.task.due_at,
        },
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
