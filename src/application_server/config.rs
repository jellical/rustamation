use crate::application_server::result::AppResult;
use config::{Config, Environment};
use serde_derive::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub database_url: String,
    pub host: String,
    pub port: String,
}

pub fn create_config() -> AppResult<AppConfig> {
    let app_config = Config::builder()
        .add_source(Environment::default())
        .build()?;

    let app_config: AppConfig = app_config.try_deserialize()?;
    Ok(app_config)
}
