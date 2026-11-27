//! Registry access token JWT di Redis; token hanya berlaku selama record aktif.
use std::time::{SystemTime, UNIX_EPOCH};

use tower_sessions_redis_store::fred::{
    interfaces::KeysInterface, prelude::Pool, types::Expiration,
};

use super::jwt::Claims;

const KEY_PREFIX: &str = "rizzshop:auth:jwt:";

#[derive(Clone)]
pub struct JwtTokenStore {
    redis: Pool,
}

impl JwtTokenStore {
    pub fn baru(redis: Pool) -> Self {
        Self { redis }
    }

    /// Menyimpan ID token di Redis sampai waktu kedaluwarsa JWT.
    pub async fn daftarkan(
        &self,
        claims: &Claims,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let ttl = claims.exp.saturating_sub(now);
        if ttl == 0 {
            return Err("JWT kedaluwarsa sebelum sempat disimpan".into());
        }
        let ttl = i64::try_from(ttl)?;

        let _: String = self
            .redis
            .set(
                Self::key(&claims.jti),
                claims.sub.to_string(),
                Some(Expiration::EX(ttl)),
                None,
                false,
            )
            .await?;
        Ok(())
    }

    /// Memastikan token sudah diterbitkan dan belum dicabut atau kedaluwarsa.
    pub async fn aktif(
        &self,
        claims: &Claims,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let user_id: Option<String> = self.redis.get(Self::key(&claims.jti)).await?;
        let expected_user_id = claims.sub.to_string();
        Ok(user_id.as_deref() == Some(expected_user_id.as_str()))
    }

    /// Mencabut satu token aktif dari Redis.
    pub async fn cabut(&self, jti: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _: i64 = self.redis.del(Self::key(jti)).await?;
        Ok(())
    }

    fn key(jti: &str) -> String {
        format!("{KEY_PREFIX}{jti}")
    }
}
