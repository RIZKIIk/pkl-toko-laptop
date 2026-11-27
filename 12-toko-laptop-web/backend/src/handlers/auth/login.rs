//! Handler autentikasi login.
use axum::{Extension, Json, extract::State, http::StatusCode, response::Response};
use serde_json::json;
use sqlx::PgPool;
use tower_sessions::Session;

use crate::auth::{self, jwt::JwtService, response, session};

use super::{payload::LoginPayload, shared::validasi_nama_pengguna};

/// Memeriksa kredensial lalu membuat session baru dan access token JWT.
pub async fn masuk(
    State(pool): State<PgPool>,
    Extension(jwt): Extension<JwtService>,
    session: Session,
    Json(payload): Json<LoginPayload>,
) -> Response {
    let username = match validasi_nama_pengguna(&payload.username) {
        Ok(username) => username,
        Err(message) => return response::respons_gagal(StatusCode::BAD_REQUEST, message),
    };
    if payload.password.is_empty() {
        return response::respons_gagal(StatusCode::BAD_REQUEST, "Password wajib diisi");
    }

    let (user_id, role) = match auth::autentikasi_pengguna(&pool, username, &payload.password).await
    {
        Ok(Some(identity)) => identity,
        Ok(None) => {
            return response::respons_gagal(
                StatusCode::UNAUTHORIZED,
                "Username atau password salah",
            );
        }
        Err(error) => {
            eprintln!("Gagal memverifikasi kredensial user: {error}");
            return response::respons_gagal(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat login",
            );
        }
    };

    let token = match jwt.terbitkan_token(user_id) {
        Ok(token) => token,
        Err(error) => {
            eprintln!("Gagal membuat JWT saat login: {error}");
            return response::respons_gagal(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat membuat token login",
            );
        }
    };

    if let Err(error) = session::mulai_sesi(&session, username).await {
        eprintln!("Gagal membuat session login: {error}");
        return response::respons_gagal(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Terjadi kesalahan pada session",
        );
    }

    response::respons_berhasil(json!({
        "status": "success",
        "message": "Login berhasil",
        "data": {
            "username": username,
            "role": role,
            "access_token": token,
            "token_type": "Bearer",
            "expires_in": auth::jwt::TOKEN_EXPIRED_SECONDS
        }
    }))
}
