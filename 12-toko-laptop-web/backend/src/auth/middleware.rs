//! Middleware autentikasi dan otorisasi request menggunakan JWT atau session.
//! Catatan: [Buka penjelasan](../../catatan/rust/auth/middleware.md)
use axum::{
    extract::{Extension, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use tower_sessions::Session;

use super::{jwt::JwtService, response::respons_gagal, role::UserRole, session::identitas_sesi};

#[derive(Clone, Debug)]
pub struct AuthIdentity {
    pub user_id: i64,
    pub username: String,
    pub role: UserRole,
}

/// Memastikan request terautentikasi dan role-nya diizinkan sebelum ke handler.
pub async fn wajibkan_autentikasi(
    State(pool): State<PgPool>,
    Extension(jwt): Extension<JwtService>,
    session: Session,
    mut request: Request,
    next: Next,
) -> Response {
    let identity = if let Some(claims) =
        super::jwt::ambil_klaim_dari_header(request.headers(), &jwt)
    {
        match crate::repositories::user_repository::cari_pengguna_berdasarkan_id(&pool, claims.sub)
            .await
        {
            Ok(Some((username, role))) => Some(AuthIdentity {
                user_id: claims.sub,
                username,
                role,
            }),
            Ok(_) => None,
            Err(error) => {
                eprintln!("Gagal memeriksa identitas JWT: {error}");
                return respons_gagal(
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
        None => match identitas_sesi(&session).await {
            Ok(Some(identity)) => {
                match crate::repositories::user_repository::cari_identitas_pengguna_berdasarkan_nama(
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
                    Ok(_) => return respons_gagal(StatusCode::UNAUTHORIZED, "Login diperlukan"),
                    Err(error) => {
                        eprintln!("Gagal memeriksa identitas session: {error}");
                        return respons_gagal(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Terjadi kesalahan saat memeriksa autentikasi",
                        );
                    }
                }
            }
            Ok(None) => return respons_gagal(StatusCode::UNAUTHORIZED, "Login diperlukan"),
            Err(error) => {
                eprintln!("Gagal membaca session: {error}");
                return respons_gagal(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Terjadi kesalahan pada session",
                );
            }
        },
    };

    if !identity
        .role
        .boleh_mengakses(request.method(), request.uri().path())
    {
        return respons_gagal(StatusCode::FORBIDDEN, "Role tidak memiliki akses");
    }

    request.extensions_mut().insert(identity);
    next.run(request).await
}
