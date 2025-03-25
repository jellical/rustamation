use config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    SQLError(#[from] sqlx::Error),
    #[error(transparent)]
    MigrationError(#[from] sqlx::migrate::MigrateError),
}
