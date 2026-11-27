//! Use case pendaftaran, verifikasi, dan sinkronisasi akun.
use std::error::Error;

use sqlx::PgPool;

use crate::repositories::user_repository;

use super::{password::buat_hash_kata_sandi, role::UserRole};

/// Menambahkan atau memperbarui akun admin dari environment dengan hash Argon2id.
pub async fn siapkan_pengguna_admin(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if verifikasi_kredensial(pool, username, password).await? {
        user_repository::ubah_peran_pengguna(pool, username, UserRole::Admin).await?;
        return Ok(());
    }

    let password_hash = buat_hash_kata_sandi(password).await?;
    user_repository::simpan_atau_perbarui_pengguna(pool, username, &password_hash, UserRole::Admin)
        .await?;
    Ok(())
}

/// Membuat akun buyer baru dengan hash password Argon2id.
pub async fn daftarkan_pengguna(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    daftarkan_pengguna_dengan_peran(pool, username, password, UserRole::Buyer).await
}

/// Membuat akun dengan role yang diberikan oleh endpoint tepercaya.
pub async fn daftarkan_pengguna_dengan_peran(
    pool: &PgPool,
    username: &str,
    password: &str,
    role: UserRole,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let password_hash = buat_hash_kata_sandi(password).await?;
    Ok(user_repository::simpan_pengguna_baru(pool, username, &password_hash, role).await?)
}

/// Memverifikasi username dan password terhadap hash Argon2id di users.
pub async fn verifikasi_kredensial(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let Some(password_hash) = user_repository::cari_hash_kata_sandi(pool, username).await? else {
        return Ok(false);
    };

    super::password::verifikasi_kata_sandi(password_hash, password).await
}

/// Memverifikasi kredensial dan mengambil identitas akun untuk menerbitkan sesi.
pub async fn autentikasi_pengguna(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<Option<(i64, UserRole)>, Box<dyn Error + Send + Sync>> {
    if !verifikasi_kredensial(pool, username, password).await? {
        return Ok(None);
    }

    Ok(user_repository::cari_identitas_pengguna_berdasarkan_nama(pool, username).await?)
}

/// Mengubah role untuk target yang sudah diizinkan oleh handler admin.
pub async fn ubah_peran_pengguna(
    pool: &PgPool,
    username: &str,
    role: UserRole,
) -> Result<bool, sqlx::Error> {
    user_repository::ubah_peran_pengguna(pool, username, role).await
}
