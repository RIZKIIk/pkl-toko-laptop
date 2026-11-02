//! Titik masuk aplikasi: memuat konfigurasi, menyiapkan database, lalu
//! menjalankan server HTTP.
//! [Buka catatan `main.rs`](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/main.md)

use std::error::Error;
// `AppConfig` membaca konfigurasi, sedangkan `routes` membuat router API.
use toko_laptop_app::{config::AppConfig, routes};

// Menyediakan runtime Tokio sehingga fungsi `main` dapat menggunakan `async`/`await`.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Memuat variabel dari file `.env` bila file tersedia.
    // Validasi nilai wajib tetap dilakukan oleh `AppConfig::from_env()`.
    dotenvy::dotenv().ok();

    // Baca konfigurasi aplikasi dari environment, lalu buat pool koneksi PostgreSQL.
    let config = AppConfig::from_env()?;
    let pool = config.connect_database().await?;

    // Terapkan migration yang belum dijalankan dari `database/migrations`.
    sqlx::migrate!("./database/migrations").run(&pool).await?;
    // Buat atau perbarui akun admin dari `ADMIN_USERNAME` dan `ADMIN_PASSWORD`.
    config.initialize_admin_user(&pool).await?;

    // Tampilkan status startup setelah koneksi, migration, dan admin berhasil disiapkan.
    println!("Koneksi PostgreSQL berhasil.");
    println!("Database migrations berhasil dijalankan.");
    println!("Akun disinkronkan dari ADMIN_USERNAME dan ADMIN_PASSWORD.");
    println!("Database: {}", config.database_name());

    // Bangun router API dengan pool database dan opsi keamanan cookie session.
    let app = routes::create_router(
        pool,
        config.session_cookie_secure,
        config.jwt_secret.clone(),
    );
    // Bentuk alamat listener dari host dan port pada konfigurasi.
    let address = (config.app_host.as_str(), config.app_port);
    // Buka port TCP; error akan dikembalikan apabila alamat tidak dapat dipakai.
    let listener = tokio::net::TcpListener::bind(address).await?;

    // Informasikan URL lokal yang dapat dipakai untuk mengakses server.
    println!(
        "Server berjalan di http://{}:{}",
        config.app_host, config.app_port
    );
    // Layani request HTTP sampai server dihentikan atau terjadi error.
    axum::serve(listener, app).await?;

    // Hanya tercapai bila server berhenti tanpa error.
    Ok(())
}
