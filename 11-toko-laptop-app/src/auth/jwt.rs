//! Pembuatan dan verifikasi access token JWT menggunakan HMAC-SHA256.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/auth/jwt.md)
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::http::{HeaderMap, header::AUTHORIZATION};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use super::role::UserRole;

/// Masa berlaku access token dalam detik (satu jam).
pub const TOKEN_EXPIRED_SECONDS: u64 = 60 * 60;

/// Service untuk menerbitkan dan memverifikasi token JWT HS256.
#[derive(Clone)]
pub struct JwtService {
    // Arc memungkinkan service mudah untuk di-clone ke setiap request.
    secret: Arc<str>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claims {
    /// ID pengguna pemilik token (`sub` = subject).
    pub sub: i64,
    /// Waktu kedaluwarsa token dalam detik sejak Unix epoch (`exp`).
    pub exp: u64,
    /// Peran pengguna untuk menentukan hak aksesnya.
    pub role: UserRole,
    /// Versi token pengguna; dibandingkan dengan versi di database agar token lama
    /// dapat ditolak setelah versi pengguna dinaikkan, misalnya saat logout.
    pub token_version: i64,
}

impl JwtService {
    /// Membuat service JWT dengan secret yang sama untuk sign dan verify.
    pub fn buat(secret: String) -> Self {
        Self {
            secret: Arc::from(secret),
        }
    }

    /// Menerbitkan token HS256 dengan identitas dan versi token pengguna.
    pub fn terbitkan_token(
        &self,
        user_id: i64,
        role: UserRole,
        token_version: i64,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // JWT memakai detik sejak Unix epoch untuk mengisi klaim expiry standar.
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let claims = Claims {
            sub: user_id,
            exp: now + TOKEN_EXPIRED_SECONDS,
            role,
            token_version,
        };
        let key = EncodingKey::from_secret(self.secret.as_bytes());

        Ok(encode(&Header::new(Algorithm::HS256), &claims, &key)?)
    }

    /// Memverifikasi signature dan expiry, lalu mengembalikan claims pengguna.
    pub fn verifikasi_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        // Algoritma dibatasi ke HS256 dan leeway nol agar expiry berlaku tepat waktu.
        let key = DecodingKey::from_secret(self.secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 0;

        Ok(decode::<Claims>(token, &key, &validation)?.claims)
    }
}

/// Mengambil token dari header Authorization dengan format `Bearer <token>`.
pub fn ambil_token_bearer(headers: &HeaderMap) -> Option<&str> {
    let value = headers.get(AUTHORIZATION)?.to_str().ok()?;
    let (scheme, token) = value.split_once(' ')?;

    if scheme.eq_ignore_ascii_case("Bearer")
        && !token.is_empty()
        && !token.chars().any(char::is_whitespace)
    {
        Some(token)
    } else {
        None
    }
}

/// Memvalidasi bearer JWT dari header dan mengambil claims pengguna.
pub fn ambil_klaim_dari_header(headers: &HeaderMap, jwt: &JwtService) -> Option<Claims> {
    jwt.verifikasi_token(ambil_token_bearer(headers)?).ok()
}
