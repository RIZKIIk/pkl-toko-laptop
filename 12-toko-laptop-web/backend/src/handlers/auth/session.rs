//! Handler untuk membaca dan menghapus sesi autentikasi.
use axum::{Extension, response::Response};
use serde_json::json;
use tower_sessions::Session;

use crate::auth::{middleware::AuthIdentity, response, session as session_service};

/// Mengembalikan identitas dari JWT bearer atau session aktif.
pub async fn pengguna_saat_ini(Extension(identity): Extension<AuthIdentity>) -> Response {
    response::respons_berhasil(json!({
        "status": "success",
        "data": {
            "id": identity.user_id,
            "username": identity.username,
            "role": identity.role
        }
    }))
}

/// Menghapus session aktif; JWT tetap berlaku hingga mencapai waktu kedaluwarsanya.
pub async fn keluar(session: Session) -> Response {
    if let Err(error) = session_service::hapus_sesi(&session).await {
        eprintln!("Gagal menghapus session saat logout: {error}");
        return response::respons_gagal(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Terjadi kesalahan pada session",
        );
    }

    response::respons_berhasil(json!({
        "status": "success",
        "message": "Logout berhasil"
    }))
}
