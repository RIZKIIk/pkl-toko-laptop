//! Query database khusus untuk tabel `users`.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/repositories/user-repository.md)
use sqlx::PgPool;

use crate::auth::role::UserRole;

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

/// Mengambil ID dan role aktif untuk username session.
pub async fn find_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<(i64, UserRole)>, sqlx::Error> {
    let user: Option<(i64, String)> =
        sqlx::query_as("SELECT id, role FROM public.users WHERE username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await?;

    user.map(|(user_id, role)| {
        role.parse::<UserRole>()
            .map_err(|error| {
                sqlx::Error::Decode(
                    std::io::Error::new(std::io::ErrorKind::InvalidData, error).into(),
                )
            })
            .map(|role| (user_id, role))
    })
    .transpose()
}

/// Mengambil ID, role, dan versi token untuk membuat JWT saat login.
pub async fn find_login_identity(
    pool: &PgPool,
    username: &str,
) -> Result<Option<(i64, UserRole, i64)>, sqlx::Error> {
    let user = sqlx::query_as::<_, (i64, String, i64)>(
        "SELECT id, role, token_version FROM public.users WHERE username = $1",
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    user.map(|(user_id, role, token_version)| {
        role.parse::<UserRole>()
            .map(|role| (user_id, role, token_version))
            .map_err(|error| {
                sqlx::Error::Decode(
                    std::io::Error::new(std::io::ErrorKind::InvalidData, error).into(),
                )
            })
    })
    .transpose()
}

/// Mengambil username, role, dan versi token berdasarkan ID pengguna.
pub async fn find_user_by_id(
    pool: &PgPool,
    user_id: i64,
) -> Result<Option<(String, UserRole, i64)>, sqlx::Error> {
    let user = sqlx::query_as::<_, (String, String, i64)>(
        "SELECT username, role, token_version FROM public.users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    user.map(|(username, role, token_version)| {
        role.parse::<UserRole>()
            .map(|role| (username, role, token_version))
            .map_err(|error| {
                sqlx::Error::Decode(
                    std::io::Error::new(std::io::ErrorKind::InvalidData, error).into(),
                )
            })
    })
    .transpose()
}

/// Mencabut seluruh JWT pengguna dengan menaikkan versi yang tersimpan.
pub async fn increment_token_version(pool: &PgPool, user_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE public.users SET token_version = token_version + 1 WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Mengubah role; middleware menolak JWT yang membawa role berbeda.
pub async fn set_user_role(
    pool: &PgPool,
    username: &str,
    role: UserRole,
) -> Result<bool, sqlx::Error> {
    let updated_username: Option<String> = sqlx::query_scalar(
        "UPDATE public.users AS existing_user \
         SET role = $2, \
             token_version = existing_user.token_version + \
                 CASE WHEN existing_user.role IS DISTINCT FROM $2 THEN 1 ELSE 0 END \
         WHERE existing_user.username = $1 \
         RETURNING existing_user.username",
    )
    .bind(username)
    .bind(role.as_str())
    .fetch_optional(pool)
    .await?;

    Ok(updated_username.is_some())
}

/// Memasukkan atau menyinkronkan akun latihan dengan nilai environment.
pub async fn upsert_user(
    pool: &PgPool,
    username: &str,
    password_hash: &str,
    role: UserRole,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO public.users AS existing_user (username, password_hash, role) \
         VALUES ($1, $2, $3) \
         ON CONFLICT (username) DO UPDATE \
         SET password_hash = EXCLUDED.password_hash, \
             role = EXCLUDED.role, \
             token_version = existing_user.token_version + \
                 CASE WHEN existing_user.password_hash IS DISTINCT FROM EXCLUDED.password_hash \
                      THEN 1 ELSE 0 END",
    )
    .bind(username)
    .bind(password_hash)
    .bind(role.as_str())
    .execute(pool)
    .await?;

    Ok(())
}

/// Membuat akun baru; `false` berarti username sudah digunakan.
pub async fn insert_user(
    pool: &PgPool,
    username: &str,
    password_hash: &str,
    role: UserRole,
) -> Result<bool, sqlx::Error> {
    // ON CONFLICT menghindari race antara pengecekan username dan proses insert.
    let inserted_username: Option<String> = sqlx::query_scalar(
        "INSERT INTO public.users (username, password_hash, role) VALUES ($1, $2, $3) \
         ON CONFLICT (username) DO NOTHING RETURNING username",
    )
    .bind(username)
    .bind(password_hash)
    .bind(role.as_str())
    .fetch_optional(pool)
    .await?;

    Ok(inserted_username.is_some())
}
