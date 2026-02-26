pub mod task;

pub mod error;

pub mod user;

pub mod worker;

use sqlx::PgPool;
use std::sync::Arc;

pub mod config;
use config::Config;

#[derive(Clone)]
pub struct ApiContext {
    pub db: PgPool,
    pub config: Arc<Config>,
}
