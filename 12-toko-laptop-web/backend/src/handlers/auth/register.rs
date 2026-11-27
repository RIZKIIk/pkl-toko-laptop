//! Handler registrasi akun publik.
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use sqlx::PgPool;

use crate::{auth, auth::response, auth::role::UserRole};

use super::{payload::RegisterPayload, shared::validasi_nama_pengguna};

/// Membuat akun baru dari username dan password yang dikirim sebagai JSON.
pub async fn daftar(State(pool): State<PgPool>, Json(payload): Json<RegisterPayload>) -> Response {
    let username = match validasi_nama_pengguna(&payload.username) {
        Ok(username) => username,
        Err(message) => return response::respons_gagal(StatusCode::BAD_REQUEST, message),
    };
    if payload.password.is_empty() {
        return response::respons_gagal(StatusCode::BAD_REQUEST, "Password wajib diisi");
    }

    match auth::daftarkan_pengguna(&pool, username, &payload.password).await {
        Ok(true) => (
            StatusCode::CREATED,
            Json(json!({
                "status": "success",
                "message": "Akun berhasil dibuat",
                "data": { "username": username, "role": UserRole::Buyer }
            })),
        )
            .into_response(),
        Ok(false) => response::respons_gagal(StatusCode::CONFLICT, "Username sudah digunakan"),
        Err(error) => {
            eprintln!("Gagal membuat akun baru: {error}");
            response::respons_gagal(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat membuat akun",
            )
        }
    }
}
