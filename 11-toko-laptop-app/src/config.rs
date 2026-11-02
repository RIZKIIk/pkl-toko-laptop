//! Membaca konfigurasi aplikasi dan membangun koneksi pool PostgreSQL.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/config.md)
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{env, error::Error, time::Duration};

/// Menyimpan pengaturan server dan koneksi PostgreSQL.
pub struct AppConfig {
    /// Host yang dipakai listener HTTP; default lokalnya `127.0.0.1`.
    pub app_host: String,
    /// Port listener HTTP dari environment variable `APP_PORT`.
    pub app_port: u16,
    /// Apakah cookie session hanya dikirim melalui HTTPS.
    pub session_cookie_secure: bool,
    /// Secret HMAC JWT; wajib sedikitnya 32 byte dan tidak boleh dicatat ke log.
    pub jwt_secret: String,
    /// Host server PostgreSQL.
    db_host: String,
    /// Port server PostgreSQL.
    db_port: u16,
    /// Nama database aplikasi.
    db_name: String,
    /// Pengguna yang dipakai untuk koneksi PostgreSQL.
    db_user: String,
    /// Kata sandi koneksi PostgreSQL.
    db_password: String,
    /// Nama pengguna admin untuk login API.
    admin_username: String,
    /// Kata sandi admin untuk login API.
    admin_password: String,
}

impl AppConfig {
    /// Membaca pengaturan dari environment variable.
    /// Hanya `APP_HOST` dan `SESSION_COOKIE_SECURE` memiliki default; port,
    /// database, akun admin, dan secret JWT wajib tersedia dan valid.
    pub fn from_env() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let session_cookie_secure = match env::var("SESSION_COOKIE_SECURE") {
            Ok(value) => value.parse()?,
            Err(env::VarError::NotPresent) => false,
            Err(error) => return Err(error.into()),
        };
        let jwt_secret = required_non_empty_env("JWT_SECRET")?;
        if jwt_secret.len() < 32 {
            return Err("JWT_SECRET harus memiliki panjang minimal 32 byte".into());
        }

        Ok(Self {
            app_host: env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            app_port: env::var("APP_PORT")?.parse()?,
            session_cookie_secure,
            jwt_secret,
            db_host: env::var("DB_HOST")?,
            db_port: env::var("DB_PORT")?.parse()?,
            db_name: env::var("DB_NAME")?,
            db_user: env::var("DB_USER")?,
            db_password: env::var("DB_PASSWORD")?,
            admin_username: required_non_empty_env("ADMIN_USERNAME")?,
            admin_password: required_non_empty_env("ADMIN_PASSWORD")?,
        })
    }

    /// Menyinkronkan akun latihan dari konfigurasi environment ke tabel users.
    pub async fn initialize_admin_user(
        &self,
        pool: &sqlx::PgPool,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        crate::auth::initialize_admin_user(pool, &self.admin_username, &self.admin_password).await
    }

    /// Membuat pool koneksi PostgreSQL untuk dipakai handler.
    /// Pool membatasi lima koneksi dan menunggu maksimal sepuluh detik untuk
    /// memperoleh koneksi sebelum mengembalikan error SQLx.
    pub async fn connect_database(&self) -> Result<sqlx::PgPool, sqlx::Error> {
        let options = PgConnectOptions::new()
            .host(&self.db_host)
            .port(self.db_port)
            .database(&self.db_name)
            .username(&self.db_user)
            .password(&self.db_password);

        PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(10))
            .connect_with(options)
            .await
    }

    /// Mengembalikan nama database untuk log startup.
    pub fn database_name(&self) -> &str {
        &self.db_name
    }
}

/// Mengambil variabel wajib dan menolak nilai kosong.
fn required_non_empty_env(name: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let value = env::var(name)?;
    if value.is_empty() {
        return Err(format!("{name} tidak boleh kosong").into());
    }
    Ok(value)
}
