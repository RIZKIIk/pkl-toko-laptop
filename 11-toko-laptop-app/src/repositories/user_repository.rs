//! Query database khusus untuk tabel `users`.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/repositories/user-repository.md)
use sqlx::PgPool;

use crate::auth::role::UserRole;

/// Mengambil hash password yang tersimpan untuk username.
pub async fn cari_hash_kata_sandi(
    pool: &PgPool,
    username: &str,
) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar("SELECT password_hash FROM public.users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
}

/// Mengambil password lama yang belum di-hash untuk proses upgrade startup.
pub async fn cari_kata_sandi_lama(pool: &PgPool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, legacy_password FROM public.users \
         WHERE password_hash IS NULL AND legacy_password IS NOT NULL ORDER BY id",
    )
    .fetch_all(pool)
    .await
}

/// Memeriksa apakah upgrade skema password lama masih perlu diselesaikan.
pub async fn punya_kolom_kata_sandi_lama(pool: &PgPool) -> Result<bool, sqlx::Error> {
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
pub async fn perbarui_hash_kata_sandi_lama(
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
pub async fn tuntaskan_migrasi_hash_kata_sandi(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("ALTER TABLE public.users ALTER COLUMN password_hash SET NOT NULL")
        .execute(pool)
        .await?;
    sqlx::query("ALTER TABLE public.users DROP COLUMN legacy_password")
        .execute(pool)
        .await?;

    Ok(())
}

/// Mengambil ID dan role aktif untuk username session.
pub async fn cari_pengguna_berdasarkan_nama(
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
pub async fn cari_identitas_pengguna_untuk_login(
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
pub async fn cari_pengguna_berdasarkan_id(
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
pub async fn naikkan_versi_token(pool: &PgPool, user_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE public.users SET token_version = token_version + 1 WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Mengubah role; middleware menolak JWT yang membawa role berbeda.
pub async fn ubah_peran_pengguna(
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
    .bind(role.sebagai_teks())
    .fetch_optional(pool)
    .await?;

    Ok(updated_username.is_some())
}

/// Memasukkan atau menyinkronkan akun latihan dengan nilai environment.
pub async fn simpan_atau_perbarui_pengguna(
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
    .bind(role.sebagai_teks())
    .execute(pool)
    .await?;

    Ok(())
}

/// Membuat akun baru; `false` berarti username sudah digunakan.
pub async fn simpan_pengguna_baru(
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
    .bind(role.sebagai_teks())
    .fetch_optional(pool)
    .await?;

    Ok(inserted_username.is_some())
}
