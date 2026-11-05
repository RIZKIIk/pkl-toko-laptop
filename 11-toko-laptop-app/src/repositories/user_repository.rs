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

/// Mengambil role aktif untuk username session.
pub async fn find_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<UserRole>, sqlx::Error> {
    let user: Option<String> =
        sqlx::query_scalar("SELECT role FROM public.users WHERE username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await?;

    user.map(|role| {
        role.parse::<UserRole>().map_err(|error| {
            sqlx::Error::Decode(std::io::Error::new(std::io::ErrorKind::InvalidData, error).into())
        })
    })
    .transpose()
}

/// Mengambil ID dan role untuk membuat kredensial login.
pub async fn find_login_identity(
    pool: &PgPool,
    username: &str,
) -> Result<Option<(i64, UserRole)>, sqlx::Error> {
    let user =
        sqlx::query_as::<_, (i64, String)>("SELECT id, role FROM public.users WHERE username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await?;

    user.map(|(user_id, role)| {
        role.parse::<UserRole>()
            .map(|role| (user_id, role))
            .map_err(|error| {
                sqlx::Error::Decode(
                    std::io::Error::new(std::io::ErrorKind::InvalidData, error).into(),
                )
            })
    })
    .transpose()
}

/// Mengambil username dan role aktif berdasarkan ID pengguna.
pub async fn find_user_by_id(
    pool: &PgPool,
    user_id: i64,
) -> Result<Option<(String, UserRole)>, sqlx::Error> {
    let user = sqlx::query_as::<_, (String, String)>(
        "SELECT username, role FROM public.users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    user.map(|(username, role)| {
        role.parse::<UserRole>()
            .map(|role| (username, role))
            .map_err(|error| {
                sqlx::Error::Decode(
                    std::io::Error::new(std::io::ErrorKind::InvalidData, error).into(),
                )
            })
    })
    .transpose()
}

/// Mengubah role; middleware menolak JWT yang membawa role berbeda.
pub async fn set_user_role(
    pool: &PgPool,
    username: &str,
    role: UserRole,
) -> Result<bool, sqlx::Error> {
    let updated_username: Option<String> = sqlx::query_scalar(
        "UPDATE public.users \
         SET role = $2 \
         WHERE username = $1 \
         RETURNING username",
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
        "INSERT INTO public.users (username, password_hash, role) VALUES ($1, $2, $3) \
         ON CONFLICT (username) DO UPDATE \
         SET password_hash = EXCLUDED.password_hash, role = EXCLUDED.role",
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
