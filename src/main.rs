use axum::{
    Router,
    Extension,
};
use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

mod config;
use config::Config;

use task_management_system::task;
use task_management_system::error;
use task_management_system::user;
use task_management_system::ApiContext;

#[tokio::main]
async fn main() ->anyhow::Result<()> {

    dotenv::dotenv().ok();

    env_logger::init();

    let config = Config::parse();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .context("Error, failed to connect")?;

    sqlx::migrate!().run(&db).await?;
    
    let api_context = ApiContext{db};

    //Create router
    let router = Router::new()
        .nest("/api/task",task::router())
        .nest("/api/users",user::router())
        .layer(Extension(api_context));

    //Define IP and Port
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener,router).await.unwrap();

    Ok(())
}



