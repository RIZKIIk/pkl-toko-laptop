//! Pembentuk response HTTP yang dipakai oleh flow autentikasi.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/auth/http.md)
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::{Value, json};

/// Membuat response error JSON dengan status yang ditentukan pemanggil.
pub(crate) fn error_response(status: StatusCode, message: &'static str) -> Response {
    (
        status,
        Json::<Value>(json!({
            "status": "error",
            "message": message
        })),
    )
        .into_response()
}

/// Membuat response sukses JSON dengan status 200.
pub(crate) fn success_response(body: Value) -> Response {
    (StatusCode::OK, Json(body)).into_response()
}
