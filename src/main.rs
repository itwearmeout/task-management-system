use axum::{
    Router,
    routing::{get},
};
use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

mod config;
use config::Config;


mod task;

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
    
    //Create router
    let router01 = Router::new()
        .route("/task",get(task::task_get).post(task::task_post));

    //Define IP and Port
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener,router01).await.unwrap();

    Ok(())
}



