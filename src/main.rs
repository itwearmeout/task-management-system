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
    let router = Router::new()
        .nest("/task",task::router());

    //Define IP and Port
    let address = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener,router).await.unwrap();

    Ok(())
}



