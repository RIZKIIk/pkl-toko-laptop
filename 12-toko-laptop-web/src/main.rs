//! Titik masuk utama aplikasi Toko Laptop.
//! Dokumentasi lengkap: file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/main.md

// Dependensi untuk menangani error, konfigurasi aplikasi, dan rute HTTP.
use std::error::Error;
use toko_laptop_app::{config::AppConfig, routes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Muat variabel lingkungan agar konfigurasi dapat dibaca dari file .env.
    dotenvy::dotenv().ok();

    // Baca konfigurasi aplikasi, lalu buka koneksi ke database.
    let config = AppConfig::baca_dari_env()?;
    let pool = config.hubungkan_ke_database().await?;

    // Siapkan daftar migrasi dan temukan migrasi baseline versi 1.
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
        .expect("migrasi baseline versi 1 harus tersedia");

    // Buang riwayat lama hanya jika checksum baseline sudah berbeda.
    if has_migration_history {
        // Riwayat yang cocok dipertahankan bersama migration baru sesudahnya.
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

        if !baseline_is_applied {
            // Hapus seluruh catatan lama agar migrasi dapat diterapkan dari baseline.
            sqlx::query("DELETE FROM public._sqlx_migrations")
                .execute(&pool)
                .await?;
        }
    }

    // Jalankan migrasi, perbarui kata sandi lama, dan pastikan akun admin tersedia.
    let migrator = sqlx::migrate::Migrator::with_migrations(migrations);
    migrator.run(&pool).await?;
    toko_laptop_app::auth::migrasikan_kata_sandi_lama(&pool).await?;
    config.siapkan_pengguna_admin(&pool).await?;

    // Tampilkan status inisialisasi aplikasi dan database.
    println!("Koneksi PostgreSQL berhasil.");
    println!("Database migrations berhasil dijalankan.");
    println!("Akun disinkronkan dari ADMIN_USERNAME dan ADMIN_PASSWORD.");
    println!("Database: {}", config.nama_database());

    // Bangun router dengan koneksi database dan pengaturan autentikasi.
    let app = routes::buat_router(
        pool,
        config.session_cookie_secure,
        config.jwt_secret.clone(),
    );

    // Buka listener pada alamat dan port yang ditentukan di konfigurasi.
    let address = (config.app_host.as_str(), config.app_port);
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!(
        "Server berjalan di http://{}:{}",
        config.app_host, config.app_port
    );

    // Jalankan server HTTP hingga terjadi error atau proses dihentikan.
    axum::serve(listener, app).await?;

    Ok(())
}
