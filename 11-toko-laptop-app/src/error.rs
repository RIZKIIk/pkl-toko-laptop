//! Mengubah kesalahan aplikasi menjadi respons JSON dengan status HTTP yang sesuai.
//! [Buka catatan `error.rs`](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/error.md)
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

/// Jenis kesalahan API beserta informasi yang diperlukan untuk response.
pub enum ApiError {
    /// Request berisi input yang tidak valid.
    BadRequest(String),
    /// Laptop yang diminta tidak ditemukan.
    NotFound(String),
    /// Query atau koneksi database mengalami kegagalan.
    Database(sqlx::Error),
}

/// Mengubah error SQLx agar bisa diteruskan sebagai `ApiError` dengan `?`.
impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl IntoResponse for ApiError {
    /// Memetakan jenis error ke status HTTP dan body JSON.
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Self::NotFound(message) => (StatusCode::NOT_FOUND, message),
            Self::Database(error) => {
                eprintln!("Kesalahan database: {error}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Terjadi kesalahan pada database".to_string(),
                )
            }
        };

        (
            status,
            Json(json!({
                "status": "error",
                "message": message
            })),
        )
            .into_response()
    }
}
