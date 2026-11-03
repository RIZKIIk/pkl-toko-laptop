//! Middleware untuk mengizinkan request dengan session login atau JWT valid.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/auth/session.md)
use axum::{
    extract::{Extension, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use tower_sessions::Session;

use super::{http::error_response, jwt::JwtService};

/// Nama key yang menyimpan username di dalam session server-side.
pub(crate) const SESSION_USERNAME_KEY: &str = "username";
pub(crate) const SESSION_AUTH_VERSION_KEY: &str = "auth_version";

#[derive(Clone, Debug)]
pub struct AuthIdentity {
    pub username: String,
    pub auth_version: i64,
}

/// Ganti ID session setelah login lalu simpan identitas pengguna.
pub async fn start_session(
    session: &Session,
    username: &str,
    auth_version: i64,
) -> Result<(), tower_sessions::session::Error> {
    // Mengganti ID mencegah session pra-login dipakai ulang setelah autentikasi.
    session.cycle_id().await?;
    session.insert(SESSION_USERNAME_KEY, username).await?;
    session.insert(SESSION_AUTH_VERSION_KEY, auth_version).await
}

/// Membaca identitas dan versi autentikasi yang disimpan dalam session.
pub(crate) async fn session_identity(
    session: &Session,
) -> Result<Option<AuthIdentity>, tower_sessions::session::Error> {
    let Some(username) = session.get(SESSION_USERNAME_KEY).await? else {
        return Ok(None);
    };
    let Some(auth_version) = session.get(SESSION_AUTH_VERSION_KEY).await? else {
        return Ok(None);
    };

    Ok(Some(AuthIdentity {
        username,
        auth_version,
    }))
}

/// Menghapus session di store dan membersihkan data session lokal.
pub async fn clear_session(session: &Session) -> Result<(), tower_sessions::session::Error> {
    session.flush().await
}

/// Memastikan request memiliki session atau JWT valid sebelum ke handler.
pub async fn require_auth(
    State(pool): State<PgPool>,
    Extension(jwt): Extension<JwtService>,
    session: Session,
    mut request: Request,
    next: Next,
) -> Response {
    let jwt_identity = if let Some(claims) =
        super::jwt::claims_from_headers(request.headers(), &jwt)
    {
        match crate::repositories::user_repository::find_auth_version(&pool, &claims.sub).await {
            Ok(Some(auth_version)) if auth_version == claims.ver => Some(AuthIdentity {
                username: claims.sub,
                auth_version,
            }),
            Ok(_) => None,
            Err(error) => {
                eprintln!("Gagal memeriksa versi autentikasi JWT: {error}");
                return error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Terjadi kesalahan saat memeriksa autentikasi",
                );
            }
        }
    } else {
        None
    };

    if let Some(identity) = jwt_identity {
        request.extensions_mut().insert(identity);
        return next.run(request).await;
    }

    match session_identity(&session).await {
        Ok(Some(identity)) => {
            match crate::repositories::user_repository::find_auth_version(&pool, &identity.username)
                .await
            {
                Ok(Some(auth_version)) if auth_version == identity.auth_version => {
                    request.extensions_mut().insert(identity);
                    next.run(request).await
                }
                Ok(_) => error_response(StatusCode::UNAUTHORIZED, "Login diperlukan"),
                Err(error) => {
                    eprintln!("Gagal memeriksa versi autentikasi session: {error}");
                    error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Terjadi kesalahan saat memeriksa autentikasi",
                    )
                }
            }
        }
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

    use super::{clear_session, session_identity, start_session};

    #[tokio::test]
    async fn session_helpers_start_read_and_clear_login() {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(None, store, None);

        start_session(&session, "admin", 3).await.unwrap();
        session.save().await.unwrap();
        assert_eq!(
            session_identity(&session)
                .await
                .unwrap()
                .map(|identity| (identity.username, identity.auth_version)),
            Some(("admin".to_owned(), 3))
        );

        clear_session(&session).await.unwrap();
        assert!(session_identity(&session).await.unwrap().is_none());
    }
}
