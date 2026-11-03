//! Pembuatan dan verifikasi access token JWT menggunakan HMAC-SHA256.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/auth/jwt.md)
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::http::{HeaderMap, header::AUTHORIZATION};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

/// Masa berlaku access token dalam detik (satu jam).
pub const TOKEN_EXPIRED_SECONDS: u64 = 60 * 60;

/// Service untuk menerbitkan dan memverifikasi token JWT HS256.
#[derive(Clone)]
pub struct JwtService {
    // Arc memungkinkan service murah untuk di-clone ke setiap request.
    secret: Arc<str>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Claims {
    pub(crate) sub: String,
    pub(crate) exp: u64,
    pub(crate) ver: i64,
}

impl JwtService {
    /// Membuat service JWT dengan secret yang sama untuk sign dan verify.
    pub fn new(secret: String) -> Self {
        Self {
            secret: Arc::from(secret),
        }
    }

    /// Menerbitkan token HS256 dengan username sebagai subject dan expiry satu jam.
    pub fn issue_token(
        &self,
        username: &str,
        auth_version: i64,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // JWT memakai detik sejak Unix epoch untuk mengisi klaim expiry standar.
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let claims = Claims {
            sub: username.to_owned(),
            exp: now + TOKEN_EXPIRED_SECONDS,
            ver: auth_version,
        };
        let key = EncodingKey::from_secret(self.secret.as_bytes());

        Ok(encode(&Header::new(Algorithm::HS256), &claims, &key)?)
    }

    /// Memverifikasi signature dan expiry, lalu mengembalikan username subject.
    pub(crate) fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        // Algoritma dibatasi ke HS256 dan leeway nol agar expiry berlaku tepat waktu.
        let key = DecodingKey::from_secret(self.secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 0;

        Ok(decode::<Claims>(token, &key, &validation)?.claims)
    }
}

/// Mengambil token dari header Authorization dengan format `Bearer <token>`.
pub(crate) fn bearer_token(headers: &HeaderMap) -> Option<&str> {
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

/// Memvalidasi bearer JWT dari header dan mengambil username di dalamnya.
pub(crate) fn claims_from_headers(headers: &HeaderMap, jwt: &JwtService) -> Option<Claims> {
    jwt.verify_token(bearer_token(headers)?).ok()
}

#[cfg(test)]
mod tests {
    use super::JwtService;

    const SECRET: &str = "test-secret-with-at-least-32-bytes-long";

    #[test]
    fn token_round_trip_returns_the_username() {
        let jwt = JwtService::new(SECRET.to_owned());
        let token = jwt.issue_token("admin", 0).unwrap();

        let claims = jwt.verify_token(&token).unwrap();
        assert_eq!(claims.sub, "admin");
        assert_eq!(claims.ver, 0);
    }

    #[test]
    fn token_rejects_tampering_and_a_different_secret() {
        let jwt = JwtService::new(SECRET.to_owned());
        let other_jwt = JwtService::new("another-test-secret-with-32-bytes".to_owned());
        let token = jwt.issue_token("admin", 0).unwrap();
        let tampered = format!("{token}x");

        assert!(jwt.verify_token(&tampered).is_err());
        assert!(other_jwt.verify_token(&token).is_err());
    }
}
