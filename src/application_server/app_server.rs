use crate::application_server::config::AppConfig;
use crate::application_server::result::AppResult;
use crate::rest_api::get_base_route;
use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn app_server(app_config: Arc<AppConfig>) -> AppResult<()> {
    let listener = TcpListener::bind(format!("{}:{}", app_config.host, app_config.port)).await?;

    let router = Router::new().merge(get_base_route());

    axum::serve(listener, router).await?;

    Ok(())
}
