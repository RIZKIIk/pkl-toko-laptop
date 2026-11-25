//! Membaca konfigurasi aplikasi dan membangun koneksi pool PostgreSQL.
//! Catatan: [Buka penjelasan](../catatan/rust/config.md)
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{env, error::Error, time::Duration};

/// Menyimpan pengaturan server, koneksi PostgreSQL, dan URL Redis.
pub struct AppConfig {
    /// Host yang dipakai listener HTTP; default lokalnya `127.0.0.1`.
    pub app_host: String,
    /// Port listener HTTP dari environment variable `APP_PORT`.
    pub app_port: u16,
    /// Apakah cookie session hanya dikirim melalui HTTPS.
    pub session_cookie_secure: bool,
    /// Secret HMAC JWT; wajib sedikitnya 32 byte dan tidak boleh dicatat ke log.
    pub jwt_secret: String,
    /// URL koneksi Redis untuk session server-side.
    pub redis_url: String,
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
    /// database, akun admin, secret JWT, dan URL Redis wajib tersedia dan valid.
    pub fn baca_dari_env() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let session_cookie_secure = match env::var("SESSION_COOKIE_SECURE") {
            Ok(value) => value.parse()?,
            Err(env::VarError::NotPresent) => false,
            Err(error) => return Err(error.into()),
        };
        let jwt_secret = ambil_env_wajib("JWT_SECRET")?;
        if jwt_secret.len() < 32 {
            return Err("JWT_SECRET harus memiliki panjang minimal 32 byte".into());
        }

        Ok(Self {
            app_host: env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            app_port: env::var("APP_PORT")?.parse()?,
            session_cookie_secure,
            jwt_secret,
            redis_url: ambil_env_wajib("REDIS_URL")?,
            db_host: env::var("DB_HOST")?,
            db_port: env::var("DB_PORT")?.parse()?,
            db_name: env::var("DB_NAME")?,
            db_user: env::var("DB_USER")?,
            db_password: env::var("DB_PASSWORD")?,
            admin_username: ambil_env_wajib("ADMIN_USERNAME")?,
            admin_password: ambil_env_wajib("ADMIN_PASSWORD")?,
        })
    }

    /// Menyinkronkan akun admin dari konfigurasi environment ke tabel users.
    pub async fn siapkan_pengguna_admin(
        &self,
        pool: &sqlx::PgPool,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        crate::auth::siapkan_pengguna_admin(pool, &self.admin_username, &self.admin_password).await
    }

    /// Membuat pool koneksi PostgreSQL untuk dipakai handler.
    /// Pool membatasi lima koneksi dan menunggu maksimal sepuluh detik untuk
    /// memperoleh koneksi sebelum mengembalikan error SQLx.
    pub async fn hubungkan_ke_database(&self) -> Result<sqlx::PgPool, sqlx::Error> {
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
    pub fn nama_database(&self) -> &str {
        &self.db_name
    }
}

/// Mengambil variabel wajib dan menolak nilai kosong.
fn ambil_env_wajib(name: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let value = env::var(name)?;
    if value.is_empty() {
        return Err(format!("{name} tidak boleh kosong").into());
    }
    Ok(value)
}
