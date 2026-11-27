//! Handler autentikasi login.
use axum::{Extension, Json, extract::State, http::StatusCode, response::Response};
use serde_json::json;
use sqlx::PgPool;
use tower_sessions::Session;

use crate::auth::{self, jwt::JwtService, response, session, token_store::JwtTokenStore};

use super::{payload::LoginPayload, shared::validasi_nama_pengguna};

/// Memeriksa kredensial lalu membuat session baru dan access token JWT.
pub async fn masuk(
    State(pool): State<PgPool>,
    Extension(jwt): Extension<JwtService>,
    Extension(token_store): Extension<JwtTokenStore>,
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

    let issued_token = match jwt.terbitkan_token(user_id) {
        Ok(token) => token,
        Err(error) => {
            eprintln!("Gagal membuat JWT saat login: {error}");
            return response::respons_gagal(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat membuat token login",
            );
        }
    };

    if let Err(error) = token_store.daftarkan(&issued_token.claims).await {
        eprintln!("Gagal menyimpan JWT di Redis saat login: {error}");
        return response::respons_gagal(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Terjadi kesalahan saat menyimpan token login",
        );
    }

    if let Err(error) = session::mulai_sesi(&session, username, &issued_token.claims.jti).await {
        eprintln!("Gagal membuat session login: {error}");
        if let Err(revoke_error) = token_store.cabut(&issued_token.claims.jti).await {
            eprintln!("Gagal membersihkan JWT setelah session login gagal: {revoke_error}");
        }
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
            "access_token": issued_token.token,
            "token_type": "Bearer",
            "expires_in": auth::jwt::TOKEN_EXPIRED_SECONDS
        }
    }))
}
