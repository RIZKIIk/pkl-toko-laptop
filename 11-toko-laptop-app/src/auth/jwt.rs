//! Pembuatan dan verifikasi access token JWT menggunakan HMAC-SHA256.
//! [Catatan JWT](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/auth/jwt.md)
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::http::{HeaderMap, header::AUTHORIZATION};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

// 60 detik × 60 menit = 3.600 detik, yaitu masa berlaku token selama satu jam.
pub const TOKEN_TTL_SECONDS: u64 = 60 * 60;

#[derive(Clone)]
pub struct JwtService {
    secret: Arc<str>,
}

#[derive(Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: u64,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self {
            secret: Arc::from(secret),
        }
    }

    pub fn issue_token(
        &self,
        username: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let claims = Claims {
            sub: username.to_owned(),
            exp: now + TOKEN_TTL_SECONDS,
        };
        let key = EncodingKey::from_secret(self.secret.as_bytes());

        Ok(encode(&Header::new(Algorithm::HS256), &claims, &key)?)
    }

    pub fn verify_token(&self, token: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let key = DecodingKey::from_secret(self.secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 0;

        let claims = decode::<Claims>(token, &key, &validation)?.claims;
        Ok(claims.sub)
    }
}

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

pub(crate) fn username_from_headers(headers: &HeaderMap, jwt: &JwtService) -> Option<String> {
    jwt.verify_token(bearer_token(headers)?).ok()
}

#[cfg(test)]
mod tests {
    use super::JwtService;

    const SECRET: &str = "test-secret-with-at-least-32-bytes-long";

    #[test]
    fn token_round_trip_returns_the_username() {
        let jwt = JwtService::new(SECRET.to_owned());
        let token = jwt.issue_token("admin").unwrap();

        assert_eq!(jwt.verify_token(&token).unwrap(), "admin");
    }

    #[test]
    fn token_rejects_tampering_and_a_different_secret() {
        let jwt = JwtService::new(SECRET.to_owned());
        let other_jwt = JwtService::new("another-test-secret-with-32-bytes".to_owned());
        let token = jwt.issue_token("admin").unwrap();
        let tampered = format!("{token}x");

        assert!(jwt.verify_token(&tampered).is_err());
        assert!(other_jwt.verify_token(&token).is_err());
    }
}
