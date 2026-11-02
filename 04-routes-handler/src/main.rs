mod config;
mod database;
mod handlers;
mod routes;
mod server;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    config::load_env()?;
    let config = config::AppConfig::from_env()?;
    let pool = database::connect(&config).await?;
    database::verify(&pool).await?;
    println!("Koneksi ke PostgreSQL berhasil.");

    let app = routes::create_router(pool);
    server::serve(app, &config.app_host, config.app_port).await?;
    Ok(())
}
