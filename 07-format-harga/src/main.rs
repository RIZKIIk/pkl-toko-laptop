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
    sqlx::query("SELECT 1 FROM public.laptops LIMIT 1")
        .execute(&pool)
        .await
        .map_err(|error| {
            eprintln!("Tabel public.laptops belum tersedia atau tidak dapat dibaca: {error}");
            error
        })?;

    let app = routes::create_router(pool);
    server::serve(app, &config.app_host, config.app_port).await?;
    Ok(())
}
