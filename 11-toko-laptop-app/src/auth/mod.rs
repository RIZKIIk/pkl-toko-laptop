//! Layanan autentikasi: seed akun, verifikasi password, session, dan JWT.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/auth/mod.md)
pub mod http;
pub mod jwt;
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

pub use session::require_auth;

/// Menambahkan atau memperbarui akun admin dari environment dengan hash Argon2id.
pub async fn initialize_admin_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if verify_credentials(pool, username, password).await? {
        user_repository::set_user_role(pool, username, UserRole::Admin).await?;
        return Ok(());
    }

    let password_hash = hash_password(password).await?;
    user_repository::upsert_user(pool, username, &password_hash, UserRole::Admin).await?;
    Ok(())
}

/// Mengubah semua password lama menjadi hash dan membuang nilai legacy.
pub async fn migrate_legacy_passwords(pool: &PgPool) -> Result<(), Box<dyn Error + Send + Sync>> {
    const PASSWORD_MIGRATION_LOCK: i64 = 7_328_491_052;

    let mut lock_connection = pool.acquire().await?;
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(PASSWORD_MIGRATION_LOCK)
        .execute(&mut *lock_connection)
        .await?;

    let migration_result = migrate_legacy_passwords_locked(pool).await;
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

async fn migrate_legacy_passwords_locked(
    pool: &PgPool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !user_repository::has_legacy_password_column(pool).await? {
        return Ok(());
    }

    let legacy_passwords = user_repository::find_legacy_passwords(pool).await?;
    for (id, password) in legacy_passwords {
        let password_hash = hash_password(&password).await?;
        user_repository::replace_legacy_password(pool, id, &password_hash).await?;
    }

    user_repository::finalize_password_hash_migration(pool).await?;
    Ok(())
}

/// Membuat akun baru dengan hash password Argon2id.
pub async fn register_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    register_user_with_role(pool, username, password, UserRole::Buyer).await
}

/// Membuat akun baru dengan role yang diberikan oleh endpoint tepercaya.
pub async fn register_user_with_role(
    pool: &PgPool,
    username: &str,
    password: &str,
    role: UserRole,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let password_hash = hash_password(password).await?;
    Ok(user_repository::insert_user(pool, username, &password_hash, role).await?)
}

/// Memverifikasi username dan password terhadap hash Argon2id di users.
pub async fn verify_credentials(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let Some(password_hash) = user_repository::find_password_hash(pool, username).await? else {
        return Ok(false);
    };

    verify_password(password_hash, password).await
}

async fn hash_password(password: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
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

async fn verify_password(
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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn argon2_password_hash_is_salted_and_verifies_only_matching_password() {
        let first = super::hash_password("secret").await.unwrap();
        let second = super::hash_password("secret").await.unwrap();

        assert_ne!(first, second);
        assert!(first.starts_with("$argon2id$"));

        assert!(
            super::verify_password(first.clone(), "secret")
                .await
                .unwrap()
        );
        assert!(
            !super::verify_password(first.clone(), "wrong")
                .await
                .unwrap()
        );
        assert!(!super::verify_password(first, "").await.unwrap());
    }
}
