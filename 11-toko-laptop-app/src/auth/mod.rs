//! Layanan autentikasi: akun, middleware, session, response, dan JWT.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/auth/mod.md)
pub mod jwt;
pub mod middleware;
pub mod response;
pub mod role;
pub mod session;

use std::error::Error;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand_core::OsRng;
use sqlx::PgPool;

use crate::repositories::user_repository;
use role::UserRole;

pub use middleware::wajibkan_autentikasi;

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

/// Mengubah semua password lama menjadi hash dan membuang nilai legacy.
pub async fn migrasikan_kata_sandi_lama(pool: &PgPool) -> Result<(), Box<dyn Error + Send + Sync>> {
    const PASSWORD_MIGRATION_LOCK: i64 = 7_328_491_052;

    let mut lock_connection = pool.acquire().await?;
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(PASSWORD_MIGRATION_LOCK)
        .execute(&mut *lock_connection)
        .await?;

    let migration_result = migrasikan_kata_sandi_lama_dengan_lock(pool).await;
    let unlock_result = sqlx::query_scalar::<_, bool>("SELECT pg_advisory_unlock($1)")
        .bind(PASSWORD_MIGRATION_LOCK)
        .fetch_one(&mut *lock_connection)
        .await;

    match (migration_result, unlock_result) {
        (Err(migration_error), Err(unlock_error)) => Err(format!(
            "Migrasi password gagal ({migration_error}); lock PostgreSQL juga gagal dilepas ({unlock_error})"
        )
        .into()),
        (Err(error), Ok(_)) => Err(error),
        (Ok(()), Err(error)) => Err(error.into()),
        (Ok(()), Ok(true)) => Ok(()),
        (Ok(()), Ok(false)) => Err("Gagal melepas lock migrasi password".into()),
    }
}

async fn migrasikan_kata_sandi_lama_dengan_lock(
    pool: &PgPool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !user_repository::punya_kolom_kata_sandi_lama(pool).await? {
        return Ok(());
    }

    let legacy_passwords = user_repository::cari_kata_sandi_lama(pool).await?;
    for (id, password) in legacy_passwords {
        let password_hash = buat_hash_kata_sandi(&password).await?;
        user_repository::perbarui_hash_kata_sandi_lama(pool, id, &password_hash).await?;
    }

    user_repository::tuntaskan_migrasi_hash_kata_sandi(pool).await?;
    Ok(())
}

/// Membuat akun baru dengan hash password Argon2id.
pub async fn daftarkan_pengguna(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    daftarkan_pengguna_dengan_peran(pool, username, password, UserRole::Buyer).await
}

/// Membuat akun baru dengan role yang diberikan oleh endpoint tepercaya.
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

    verifikasi_kata_sandi(password_hash, password).await
}

async fn buat_hash_kata_sandi(password: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let password = password.to_owned();
    Ok(tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Ok::<_, argon2::password_hash::Error>(
            Argon2::default()
                .hash_password(password.as_bytes(), &salt)?
                .to_string(),
        )
    })
    .await??)
}

async fn verifikasi_kata_sandi(
    password_hash: String,
    password: &str,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    if password.is_empty() {
        return Ok(false);
    }

    let password = password.to_owned();
    Ok(tokio::task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&password_hash)?;
        Ok::<_, argon2::password_hash::Error>(
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok(),
        )
    })
    .await??)
}
