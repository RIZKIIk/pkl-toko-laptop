//! Titik masuk utama aplikasi Toko Laptop.
//! Dokumentasi lengkap: ../catatan/rust/main.md

// Dependensi untuk menangani error, konfigurasi aplikasi, dan rute HTTP.
use std::error::Error;
use toko_laptop_app::{config::AppConfig, database, endpoin};
use tower_sessions_redis_store::fred::{
    interfaces::ClientLike,
    prelude::{Config as RedisConfig, Pool},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Muat variabel lingkungan agar konfigurasi dapat dibaca dari file .env.
    dotenvy::dotenv().ok();

    // Baca konfigurasi aplikasi, lalu buka koneksi ke database.
    let config = AppConfig::baca_dari_env()?;
    let pool = config.hubungkan_ke_database().await?;

    // Redis wajib tersedia untuk session dan pemeriksaan registry JWT.
    let redis_config = RedisConfig::from_url(&config.redis_url)?;
    let redis_pool = Pool::new(redis_config, None, None, None, 6)?;
    let redis_connection = redis_pool.connect();
    redis_pool.wait_for_connect().await?;
    println!("Redis session store dan registry JWT berhasil terhubung.");

    // Terapkan skema, upgrade hash password, dan pastikan akun admin tersedia.
    database::jalankan_migrasi(&pool).await?;
    toko_laptop_app::auth::migrasikan_kata_sandi_lama(&pool).await?;
    config.siapkan_pengguna_admin(&pool).await?;

    // Tampilkan status inisialisasi aplikasi dan database.
    println!("Koneksi PostgreSQL berhasil.");
    println!("Database migrations berhasil dijalankan.");
    println!("Akun disinkronkan dari ADMIN_USERNAME dan ADMIN_PASSWORD.");
    println!("Database: {}", config.nama_database());

    // Bangun router dengan koneksi database dan pengaturan autentikasi.
    let app = endpoin::buat_router(
        pool,
        config.session_cookie_secure,
        config.jwt_secret.clone(),
        redis_pool,
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
    redis_connection.await??;

    Ok(())
}
