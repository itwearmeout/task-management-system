pub mod task;

pub mod error;

pub mod user;

use sqlx::PgPool;

#[derive(Clone)]
pub struct ApiContext {
    pub db: PgPool,
}