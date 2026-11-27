//! Menjalankan migrasi skema PostgreSQL sebelum server menerima request.
use std::error::Error;

use sqlx::PgPool;

/// Menerapkan migrasi SQLx dan membuang riwayat lama hanya jika baseline berubah.
pub async fn jalankan_migrasi(pool: &PgPool) -> Result<(), Box<dyn Error + Send + Sync>> {
    let has_migration_history: bool =
        sqlx::query_scalar("SELECT to_regclass('public._sqlx_migrations') IS NOT NULL")
            .fetch_one(pool)
            .await?;

    let migrations = sqlx::migrate!("./database/migrations")
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    let baseline = migrations
        .iter()
        .find(|migration| migration.version == 1)
        .expect("migrasi baseline versi 1 harus tersedia");

    if has_migration_history {
        let baseline_is_applied: bool = sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT 1 FROM public._sqlx_migrations
                WHERE version = $1 AND checksum = $2 AND success
            )",
        )
        .bind(baseline.version)
        .bind(baseline.checksum.as_ref().to_vec())
        .fetch_one(pool)
        .await?;

        if !baseline_is_applied {
            sqlx::query("DELETE FROM public._sqlx_migrations")
                .execute(pool)
                .await?;
        }
    }

    sqlx::migrate::Migrator::with_migrations(migrations)
        .run(pool)
        .await?;
    Ok(())
}
