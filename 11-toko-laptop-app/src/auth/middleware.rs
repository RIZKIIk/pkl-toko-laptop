//! Middleware autentikasi dan otorisasi request menggunakan JWT atau session.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/auth/middleware.md)
use axum::{
    extract::{Extension, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use tower_sessions::Session;

use super::{jwt::JwtService, response::error_response, role::UserRole, session::session_identity};

#[derive(Clone, Debug)]
pub struct AuthIdentity {
    pub user_id: i64,
    pub username: String,
    pub role: UserRole,
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
