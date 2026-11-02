//! Query database khusus untuk tabel `users`.
//! [Catatan repository user](../../catatan/rust/repositories/user-repository.md)
use sqlx::PgPool;

/// Mengambil password latihan yang tersimpan untuk username.
pub async fn find_password(pool: &PgPool, username: &str) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar("SELECT password FROM public.users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
}

/// Memasukkan atau menyinkronkan akun latihan dengan nilai environment.
pub async fn upsert_user(pool: &PgPool, username: &str, password: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO public.users (username, password) VALUES ($1, $2) \
         ON CONFLICT (username) DO UPDATE SET password = EXCLUDED.password",
    )
    .bind(username)
    .bind(password)
    .execute(pool)
    .await?;

    Ok(())
}

/// Membuat akun baru; `false` berarti username sudah digunakan.
pub async fn insert_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<bool, sqlx::Error> {
    let inserted_username: Option<String> = sqlx::query_scalar(
        "INSERT INTO public.users (username, password) VALUES ($1, $2) \
         ON CONFLICT (username) DO NOTHING RETURNING username",
    )
    .bind(username)
    .bind(password)
    .fetch_optional(pool)
    .await?;

    Ok(inserted_username.is_some())
}
