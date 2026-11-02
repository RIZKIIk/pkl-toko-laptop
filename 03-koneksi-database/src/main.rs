mod config;
mod database;

use std::error::Error;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    if env_path.exists() {
        dotenvy::from_path(env_path)?;
    }

    let config = config::AppConfig::from_env()?;
    let pool = database::connect(&config).await?;
    database::verify(&pool).await?;
    println!("Koneksi ke PostgreSQL berhasil.");
    println!("Database yang diuji: {}", config.database_name());
    pool.close().await;
    Ok(())
}
