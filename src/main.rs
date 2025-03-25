use crate::application_server::app_server::app_server;
use crate::application_server::config::create_config;
use crate::application_server::result::AppResult;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub mod application_server;
pub mod rest_api;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();

    let config = create_config()?;

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.database_url)
        .await?;

    println!("{:#?}", config);

    sqlx::migrate!().run(&db).await?;

    app_server(Arc::new(config)).await?;

    Ok(())
}
