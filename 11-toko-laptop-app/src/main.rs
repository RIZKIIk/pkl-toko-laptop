//! Titik masuk utama aplikasi Toko Laptop.
//! Dokumentasi lengkap: file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/main.md

use std::error::Error;
use toko_laptop_app::{config::AppConfig, routes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Memuat variabel lingkungan dari file .env.
    dotenvy::dotenv().ok();

    // Inisialisasi konfigurasi dan koneksi database.
    let config = AppConfig::from_env()?;
    let pool = config.connect_database().await?;

    // Menjalankan migrasi dan inisialisasi akun admin.
    sqlx::migrate!("./database/migrations").run(&pool).await?;
    toko_laptop_app::auth::migrate_legacy_passwords(&pool).await?;
    config.initialize_admin_user(&pool).await?;

    // Menampilkan informasi status aplikasi.
    println!("Koneksi PostgreSQL berhasil.");
    println!("Database migrations berhasil dijalankan.");
    println!("Akun disinkronkan dari ADMIN_USERNAME dan ADMIN_PASSWORD.");
    println!("Database: {}", config.database_name());

    // Membuat router aplikasi.
    let app = routes::create_router(
        pool,
        config.session_cookie_secure,
        config.jwt_secret.clone(),
    );

    // Menyiapkan alamat dan listener server.
    let address = (config.app_host.as_str(), config.app_port);
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!(
        "Server berjalan di http://{}:{}",
        config.app_host, config.app_port
    );

    // Menjalankan server HTTP menggunakan Axum.
    axum::serve(listener, app).await?;

    Ok(())
}
