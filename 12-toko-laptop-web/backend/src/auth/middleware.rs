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

use super::{
    jwt::{JwtService, ambil_token_bearer},
    response::respons_gagal,
    role::UserRole,
    session::identitas_sesi,
    token_store::JwtTokenStore,
};

#[derive(Clone, Debug)]
pub struct AuthIdentity {
    pub user_id: i64,
    pub username: String,
    pub role: UserRole,
    /// ID token bearer yang dipakai request, jika autentikasi memakai JWT.
    pub jwt_id: Option<String>,
}

/// Memastikan request terautentikasi dan role-nya diizinkan sebelum ke handler.
pub async fn wajibkan_autentikasi(
    State(pool): State<PgPool>,
    Extension(jwt): Extension<JwtService>,
    Extension(token_store): Extension<JwtTokenStore>,
    session: Session,
    mut request: Request,
    next: Next,
) -> Response {
    let identity = if let Some(token) = ambil_token_bearer(request.headers()) {
        let claims = match jwt.verifikasi_token(token) {
            Ok(claims) => claims,
            Err(_) => return respons_gagal(StatusCode::UNAUTHORIZED, "Login diperlukan"),
        };
        match token_store.aktif(&claims).await {
            Ok(true) => {}
            Ok(false) => return respons_gagal(StatusCode::UNAUTHORIZED, "Login diperlukan"),
            Err(error) => {
                eprintln!("Gagal memeriksa JWT di Redis: {error}");
                return respons_gagal(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Terjadi kesalahan saat memeriksa autentikasi",
                );
            }
        }
        match crate::repositories::user_repository::cari_pengguna_berdasarkan_id(&pool, claims.sub)
            .await
        {
            Ok(Some((username, role))) => Some(AuthIdentity {
                user_id: claims.sub,
                username,
                role,
                jwt_id: Some(claims.jti),
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
                        jwt_id: None,
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
