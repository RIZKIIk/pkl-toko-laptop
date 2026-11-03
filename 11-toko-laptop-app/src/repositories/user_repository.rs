//! Query database khusus untuk tabel `users`.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/repositories/user-repository.md)
use sqlx::PgPool;

/// Mengambil hash password yang tersimpan untuk username.
pub async fn find_password_hash(
    pool: &PgPool,
    username: &str,
) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar("SELECT password_hash FROM public.users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
}

/// Mengambil password lama yang belum di-hash untuk proses upgrade startup.
pub async fn find_legacy_passwords(pool: &PgPool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, legacy_password FROM public.users \
         WHERE password_hash IS NULL AND legacy_password IS NOT NULL ORDER BY id",
    )
    .fetch_all(pool)
    .await
}

/// Memeriksa apakah upgrade skema password lama masih perlu diselesaikan.
pub async fn has_legacy_password_column(pool: &PgPool) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT EXISTS ( \
             SELECT 1 FROM information_schema.columns \
             WHERE table_schema = 'public' \
               AND table_name = 'users' \
               AND column_name = 'legacy_password' \
         )",
    )
    .fetch_one(pool)
    .await
}

/// Mengganti password lama dengan hash tanpa menimpa hasil migrasi paralel.
pub async fn replace_legacy_password(
    pool: &PgPool,
    id: i64,
    password_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE public.users \
         SET password_hash = $2, legacy_password = NULL \
         WHERE id = $1 AND password_hash IS NULL",
    )
    .bind(id)
    .bind(password_hash)
    .execute(pool)
    .await?;

    Ok(())
}

/// Memastikan semua akun memiliki hash dan menghapus kolom legacy.
pub async fn finalize_password_hash_migration(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("ALTER TABLE public.users ALTER COLUMN password_hash SET NOT NULL")
        .execute(pool)
        .await?;
    sqlx::query("ALTER TABLE public.users DROP COLUMN legacy_password")
        .execute(pool)
        .await?;

    Ok(())
}

/// Mengambil versi autentikasi aktif untuk username.
pub async fn find_auth_version(pool: &PgPool, username: &str) -> Result<Option<i64>, sqlx::Error> {
    sqlx::query_scalar("SELECT auth_version FROM public.users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
}

/// Mencabut semua session dan JWT dengan menaikkan versi autentikasi user.
pub async fn invalidate_user_sessions(
    pool: &PgPool,
    username: &str,
) -> Result<Option<i64>, sqlx::Error> {
    sqlx::query_scalar(
        "UPDATE public.users SET auth_version = auth_version + 1 \
         WHERE username = $1 RETURNING auth_version",
    )
    .bind(username)
    .fetch_optional(pool)
    .await
}

/// Memasukkan atau menyinkronkan akun latihan dengan nilai environment.
pub async fn upsert_user(
    pool: &PgPool,
    username: &str,
    password_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO public.users (username, password_hash) VALUES ($1, $2) \
         ON CONFLICT (username) DO UPDATE \
         SET password_hash = EXCLUDED.password_hash, auth_version = users.auth_version + 1",
    )
    .bind(username)
    .bind(password_hash)
    .execute(pool)
    .await?;

    Ok(())
}

/// Membuat akun baru; `false` berarti username sudah digunakan.
pub async fn insert_user(
    pool: &PgPool,
    username: &str,
    password_hash: &str,
) -> Result<bool, sqlx::Error> {
    // ON CONFLICT menghindari race antara pengecekan username dan proses insert.
    let inserted_username: Option<String> = sqlx::query_scalar(
        "INSERT INTO public.users (username, password_hash) VALUES ($1, $2) \
         ON CONFLICT (username) DO NOTHING RETURNING username",
    )
    .bind(username)
    .bind(password_hash)
    .fetch_optional(pool)
    .await?;

    Ok(inserted_username.is_some())
}
