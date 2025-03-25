use crate::application_server::error::AppError;

pub type AppResult<T> = Result<T, AppError>;
