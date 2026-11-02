//! Middleware untuk mengizinkan request dengan session login atau JWT valid.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/auth/session.md)
use axum::{
    extract::{Extension, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tower_sessions::Session;

use super::{http::error_response, jwt::JwtService};

/// Nama key yang menyimpan username di dalam session server-side.
pub(crate) const SESSION_USERNAME_KEY: &str = "username";

/// Ganti ID session setelah login lalu simpan identitas pengguna.
pub async fn start_session(
    session: &Session,
    username: &str,
) -> Result<(), tower_sessions::session::Error> {
    // Mengganti ID mencegah session pra-login dipakai ulang setelah autentikasi.
    session.cycle_id().await?;
    session.insert(SESSION_USERNAME_KEY, username).await
}

/// Membaca username yang disimpan dalam session aktif.
pub async fn session_username(
    session: &Session,
) -> Result<Option<String>, tower_sessions::session::Error> {
    session.get(SESSION_USERNAME_KEY).await
}

/// Menghapus session di store dan membersihkan data session lokal.
pub async fn clear_session(session: &Session) -> Result<(), tower_sessions::session::Error> {
    session.flush().await
}

/// Memastikan request memiliki session atau JWT valid sebelum ke handler.
pub async fn require_auth(
    Extension(jwt): Extension<JwtService>,
    session: Session,
    request: Request,
    next: Next,
) -> Response {
    // Token bearer yang sah cukup untuk melanjutkan request tanpa membaca session.
    if super::jwt::username_from_headers(request.headers(), &jwt).is_some() {
        return next.run(request).await;
    }

    match session_username(&session).await {
        Ok(Some(_)) => next.run(request).await,
        Ok(None) => error_response(StatusCode::UNAUTHORIZED, "Login diperlukan"),
        Err(error) => {
            eprintln!("Gagal membaca session: {error}");
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan pada session",
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tower_sessions::{MemoryStore, Session};

    use super::{clear_session, session_username, start_session};

    #[tokio::test]
    async fn session_helpers_start_read_and_clear_login() {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(None, store, None);

        start_session(&session, "admin").await.unwrap();
        session.save().await.unwrap();
        assert_eq!(
            session_username(&session).await.unwrap().as_deref(),
            Some("admin")
        );

        clear_session(&session).await.unwrap();
        assert_eq!(session_username(&session).await.unwrap(), None);
    }
}
