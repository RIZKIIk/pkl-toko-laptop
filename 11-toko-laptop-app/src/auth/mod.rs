//! Layanan autentikasi: seed akun, verifikasi password, session, dan JWT.
//! [Catatan modul auth](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/auth/mod.md)
pub mod http;
pub mod jwt;
pub mod session;

use std::error::Error;

use sqlx::PgPool;

use crate::repositories::user_repository;

pub use session::require_auth;

/// Menambahkan atau memperbarui akun latihan dari env.
/// Password disimpan biasa hanya untuk latihan session lokal.
pub async fn initialize_admin_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    user_repository::upsert_user(pool, username, password).await?;
    Ok(())
}

/// Membuat akun baru tanpa menimpa akun dengan username yang sama.
pub async fn register_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<bool, sqlx::Error> {
    user_repository::insert_user(pool, username, password).await
}

/// Memverifikasi username dan password terhadap tabel users.
pub async fn verify_credentials(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let Some(stored_password) = user_repository::find_password(pool, username).await? else {
        return Ok(false);
    };

    Ok(password_matches(&stored_password, password))
}

fn password_matches(stored_password: &str, provided_password: &str) -> bool {
    !provided_password.is_empty() && stored_password == provided_password
}

#[cfg(test)]
mod tests {
    #[test]
    fn credential_check_requires_exact_non_empty_password() {
        assert!(super::password_matches("secret", "secret"));
        assert!(!super::password_matches("secret", "wrong"));
        assert!(!super::password_matches("", ""));
    }
}
