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

    // Baseline versi 1 menggantikan riwayat migration lama tanpa menghapus data aplikasi.
    let has_migration_history: bool =
        sqlx::query_scalar("SELECT to_regclass('public._sqlx_migrations') IS NOT NULL")
            .fetch_one(&pool)
            .await?;
    let migrations = sqlx::migrate!("./database/migrations")
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    let baseline = migrations
        .iter()
        .find(|migration| migration.version == 1)
        .expect("baseline migration version 1 must exist");

    if has_migration_history {
        let baseline_is_applied: bool = sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT 1 FROM public._sqlx_migrations
                WHERE version = $1 AND checksum = $2 AND success
            )",
        )
        .bind(baseline.version)
        .bind(baseline.checksum.as_ref().to_vec())
        .fetch_one(&pool)
        .await?;

        if baseline_is_applied {
            sqlx::query("DELETE FROM public._sqlx_migrations WHERE version <> $1")
                .bind(baseline.version)
                .execute(&pool)
                .await?;
        } else {
            sqlx::query("DELETE FROM public._sqlx_migrations")
                .execute(&pool)
                .await?;
        }
    }

    let migrator = sqlx::migrate::Migrator::with_migrations(migrations);
    migrator.run(&pool).await?;
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
