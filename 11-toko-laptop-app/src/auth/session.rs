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

use super::{http::error_response, jwt::JwtService, role::UserRole};

/// Nama key yang menyimpan username di dalam session server-side.
pub const SESSION_USERNAME_KEY: &str = "username";

#[derive(Clone, Debug)]
pub struct AuthIdentity {
    pub user_id: i64,
    pub username: String,
    pub role: UserRole,
}

#[derive(Clone, Debug)]
pub struct SessionIdentity {
    pub username: String,
}

/// Ganti ID session setelah login lalu simpan identitas pengguna.
pub async fn start_session(
    session: &Session,
    username: &str,
) -> Result<(), tower_sessions::session::Error> {
    // Mengganti ID mencegah session pra-login dipakai ulang setelah autentikasi.
    session.cycle_id().await?;
    session.insert(SESSION_USERNAME_KEY, username).await
}

/// Membaca username yang disimpan dalam session.
pub async fn session_identity(
    session: &Session,
) -> Result<Option<SessionIdentity>, tower_sessions::session::Error> {
    let Some(username) = session.get(SESSION_USERNAME_KEY).await? else {
        return Ok(None);
    };
    Ok(Some(SessionIdentity { username }))
}

/// Menghapus session di store dan membersihkan data session lokal.
pub async fn clear_session(session: &Session) -> Result<(), tower_sessions::session::Error> {
    session.flush().await
}

/// Memastikan request terautentikasi dan role-nya diizinkan sebelum ke handler.
pub async fn require_auth(
    State(pool): State<PgPool>,
    Extension(jwt): Extension<JwtService>,
    session: Session,
    mut request: Request,
    next: Next,
) -> Response {
    let identity = if let Some(claims) = super::jwt::claims_from_headers(request.headers(), &jwt) {
        match crate::repositories::user_repository::find_user_by_id(&pool, claims.sub).await {
            Ok(Some((username, role, token_version)))
                if role == claims.role && token_version == claims.token_version =>
            {
                Some(AuthIdentity {
                    user_id: claims.sub,
                    username,
                    role,
                })
            }
            Ok(_) => None,
            Err(error) => {
                eprintln!("Gagal memeriksa identitas JWT: {error}");
                return error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Terjadi kesalahan saat memeriksa autentikasi",
                );
            }
        }
    } else {
        None
    };

    let identity = match identity {
        Some(identity) => identity,
        None => match session_identity(&session).await {
            Ok(Some(identity)) => {
                match crate::repositories::user_repository::find_user_by_username(
                    &pool,
                    &identity.username,
                )
                .await
                {
                    Ok(Some((user_id, role))) => AuthIdentity {
                        user_id,
                        username: identity.username,
                        role,
                    },
                    Ok(_) => return error_response(StatusCode::UNAUTHORIZED, "Login diperlukan"),
                    Err(error) => {
                        eprintln!("Gagal memeriksa identitas session: {error}");
                        return error_response(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Terjadi kesalahan saat memeriksa autentikasi",
                        );
                    }
                }
            }
            Ok(None) => return error_response(StatusCode::UNAUTHORIZED, "Login diperlukan"),
            Err(error) => {
                eprintln!("Gagal membaca session: {error}");
                return error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Terjadi kesalahan pada session",
                );
            }
        },
    };

    if !identity
        .role
        .can_access(request.method(), request.uri().path())
    {
        return error_response(StatusCode::FORBIDDEN, "Role tidak memiliki akses");
    }

    request.extensions_mut().insert(identity);
    next.run(request).await
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

        start_session(&session, "admin").await.unwrap();
        session.save().await.unwrap();
        assert_eq!(
            session_identity(&session)
                .await
                .unwrap()
                .map(|identity| identity.username),
            Some("admin".to_owned())
        );

        clear_session(&session).await.unwrap();
        assert!(session_identity(&session).await.unwrap().is_none());
    }
}
