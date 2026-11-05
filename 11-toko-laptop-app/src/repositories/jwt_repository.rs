//! Query untuk mencabut JWT tertentu sampai waktu kedaluwarsanya.
use sqlx::PgPool;

/// Memeriksa apakah ID JWT sudah dicabut dan belum kedaluwarsa.
pub async fn is_revoked(pool: &PgPool, jti: &str) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT EXISTS ( \
             SELECT 1 FROM public.revoked_jwts \
             WHERE jti = $1 AND expires_at > EXTRACT(EPOCH FROM NOW())::BIGINT \
         )",
    )
    .bind(jti)
    .fetch_one(pool)
    .await
}

/// Mencatat ID JWT yang logout sampai token itu kedaluwarsa.
pub async fn revoke(pool: &PgPool, jti: &str, expires_at: u64) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM public.revoked_jwts \
         WHERE expires_at <= EXTRACT(EPOCH FROM NOW())::BIGINT",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "INSERT INTO public.revoked_jwts (jti, expires_at) VALUES ($1, $2) \
         ON CONFLICT (jti) DO NOTHING",
    )
    .bind(jti)
    .bind(expires_at as i64)
    .execute(pool)
    .await?;

    Ok(())
}
