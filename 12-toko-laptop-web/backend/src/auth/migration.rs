//! Upgrade hash akun lama sebelum server menerima request.
use std::error::Error;

use sqlx::PgPool;

use crate::repositories::user_repository;

use super::password::buat_hash_kata_sandi;

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
