//! Handler untuk login, pemeriksaan, dan logout session admin.
//! [Catatan handler auth](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/handlers/auth.md)
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use tower_sessions::Session;

use crate::auth::{self, http, session};

fn validate_username(username: &str) -> Result<&str, &'static str> {
    let username = username.trim();
    if username.is_empty() {
        return Err("Username wajib diisi");
    }
    if username.chars().count() > 100 {
        return Err("Username maksimal 100 karakter");
    }
    Ok(username)
}

/// Body JSON yang wajib dikirim ke `POST /api/auth/login`.
#[derive(Deserialize)]
// Menolak field tambahan agar body login selalu sesuai kontrak API.
#[serde(deny_unknown_fields)]
pub struct LoginPayload {
    /// Nama akun yang akan dicocokkan dengan tabel `users`.
    username: String,
    /// Password latihan yang akan diverifikasi; tidak dikirim kembali ke client.
    password: String,
}

/// Memeriksa kredensial admin dan membuat session baru.
pub async fn login(
    State(pool): State<PgPool>,
    session: Session,
    Json(payload): Json<LoginPayload>,
) -> Response {
    let username = match validate_username(&payload.username) {
        Ok(username) => username,
        Err(message) => return http::error_response(StatusCode::BAD_REQUEST, message),
    };
    if payload.password.is_empty() {
        return http::error_response(StatusCode::BAD_REQUEST, "Password wajib diisi");
    }

    let credentials_match = match auth::verify_credentials(&pool, username, &payload.password).await
    {
        Ok(matches) => matches,
        Err(error) => {
            eprintln!("Gagal memverifikasi kredensial user: {error}");
            return http::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat login",
            );
        }
    };

    if !credentials_match {
        return http::error_response(StatusCode::UNAUTHORIZED, "Username atau password salah");
    }

    if let Err(error) = session::start_session(&session, &payload.username).await {
        eprintln!("Gagal membuat session login: {error}");
        return http::error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Terjadi kesalahan pada session",
        );
    }

    http::success_response(json!({
        "status": "success",
        "message": "Login berhasil",
        "data": { "username": username }
    }))
}

/// Membuat akun baru dari username dan password yang dikirim sebagai JSON.
pub async fn register(State(pool): State<PgPool>, Json(payload): Json<LoginPayload>) -> Response {
    let username = match validate_username(&payload.username) {
        Ok(username) => username,
        Err(message) => return http::error_response(StatusCode::BAD_REQUEST, message),
    };
    if payload.password.is_empty() {
        return http::error_response(StatusCode::BAD_REQUEST, "Password wajib diisi");
    }

    match auth::register_user(&pool, username, &payload.password).await {
        Ok(true) => (
            StatusCode::CREATED,
            axum::Json(json!({
                "status": "success",
                "message": "Akun berhasil dibuat",
                "data": { "username": username }
            })),
        )
            .into_response(),
        Ok(false) => http::error_response(StatusCode::CONFLICT, "Username sudah digunakan"),
        Err(error) => {
            eprintln!("Gagal membuat akun baru: {error}");
            http::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat membuat akun",
            )
        }
    }
}

/// Mengembalikan identitas dari session aktif.
pub async fn current_user(session: Session) -> Response {
    match auth::session::session_username(&session).await {
        Ok(Some(username)) => http::success_response(json!({
            "status": "success",
            "data": { "username": username }
        })),
        Ok(None) => http::error_response(StatusCode::UNAUTHORIZED, "Login diperlukan"),
        Err(error) => {
            eprintln!("Gagal membaca session pengguna: {error}");
            http::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan pada session",
            )
        }
    }
}

/// Menghapus session aktif agar cookie tidak dapat digunakan lagi.
pub async fn logout(session: Session) -> Response {
    if let Err(error) = auth::session::clear_session(&session).await {
        eprintln!("Gagal menghapus session saat logout: {error}");
        return http::error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Terjadi kesalahan pada session",
        );
    }

    http::success_response(json!({
        "status": "success",
        "message": "Logout berhasil"
    }))
}

#[cfg(test)]
mod tests {
    use super::validate_username;

    #[test]
    fn username_is_trimmed_before_use() {
        assert_eq!(validate_username("  rizki  "), Ok("rizki"));
    }

    #[test]
    fn username_must_not_be_empty() {
        assert_eq!(validate_username("   "), Err("Username wajib diisi"));
    }

    #[test]
    fn username_cannot_exceed_database_limit() {
        assert!(validate_username(&"a".repeat(100)).is_ok());
        assert_eq!(
            validate_username(&"a".repeat(101)),
            Err("Username maksimal 100 karakter")
        );
    }
}
