mod config;
mod database;
mod error;
mod handlers;
mod models;
mod presenters;
mod repository;
mod routes;
mod server;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    config::load_env()?;
    let config = config::AppConfig::from_env()?;
    let pool = database::connect(&config).await?;
    let app = routes::create_router(pool, config.session_cookie_secure);
    server::serve(app, &config.app_host, config.app_port).await?;
    Ok(())
}
