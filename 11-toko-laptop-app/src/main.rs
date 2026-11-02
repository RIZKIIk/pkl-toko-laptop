//! Menyiapkan konfigurasi, database, lalu server HTTP.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/main.md)
use std::error::Error;
use toko_laptop_app::{config::AppConfig, routes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env()?;
    let pool = config.connect_database().await?;

    sqlx::migrate!("./database/migrations").run(&pool).await?;
    config.initialize_admin_user(&pool).await?;

    println!("Koneksi PostgreSQL berhasil.");
    println!("Database migrations berhasil dijalankan.");
    println!("Akun disinkronkan dari ADMIN_USERNAME dan ADMIN_PASSWORD.");
    println!("Database: {}", config.database_name());

    let app = routes::create_router(
        pool,
        config.session_cookie_secure,
        config.jwt_secret.clone(),
    );
    let address = (config.app_host.as_str(), config.app_port);
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!(
        "Server berjalan di http://{}:{}",
        config.app_host, config.app_port
    );
    axum::serve(listener, app).await?;

    Ok(())
}
