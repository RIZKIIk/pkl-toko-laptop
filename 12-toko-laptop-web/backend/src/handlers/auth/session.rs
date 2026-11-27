//! Handler untuk membaca dan menghapus sesi autentikasi.
use axum::{Extension, http::StatusCode, response::Response};
use serde_json::json;
use tower_sessions::Session;

use crate::auth::{
    middleware::AuthIdentity, response, session as session_service, token_store::JwtTokenStore,
};

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

/// Mencabut token JWT aktif di Redis lalu menghapus cookie session.
pub async fn keluar(
    Extension(identity): Extension<AuthIdentity>,
    Extension(token_store): Extension<JwtTokenStore>,
    session: Session,
) -> Response {
    let session_jwt_id = match session_service::identitas_sesi(&session).await {
        Ok(identity) => identity.and_then(|identity| identity.jwt_id),
        Err(error) => {
            eprintln!("Gagal membaca token ID dari session saat logout: {error}");
            return response::respons_gagal(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan pada session",
            );
        }
    };

    let mut jwt_ids = Vec::new();
    if let Some(jwt_id) = identity.jwt_id.as_deref() {
        jwt_ids.push(jwt_id);
    }
    if let Some(jwt_id) = session_jwt_id.as_deref()
        && !jwt_ids.contains(&jwt_id)
    {
        jwt_ids.push(jwt_id);
    }

    for jwt_id in jwt_ids {
        if let Err(error) = token_store.cabut(jwt_id).await {
            eprintln!("Gagal mencabut JWT saat logout: {error}");
            return response::respons_gagal(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat mengakhiri token",
            );
        }
    }

    if let Err(error) = session_service::hapus_sesi(&session).await {
        eprintln!("Gagal menghapus session saat logout: {error}");
        return response::respons_gagal(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Terjadi kesalahan pada session",
        );
    }

    response::respons_berhasil(json!({
        "status": "success",
        "message": "Logout berhasil"
    }))
}
