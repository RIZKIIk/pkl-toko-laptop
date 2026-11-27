//! Hash dan verifikasi password memakai Argon2id.
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand_core::OsRng;

pub(super) async fn buat_hash_kata_sandi(
    password: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let password = password.to_owned();
    Ok(tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Ok::<_, argon2::password_hash::Error>(
            Argon2::default()
                .hash_password(password.as_bytes(), &salt)?
                .to_string(),
        )
    })
    .await??)
}

pub(super) async fn verifikasi_kata_sandi(
    password_hash: String,
    password: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    if password.is_empty() {
        return Ok(false);
    }

    let password = password.to_owned();
    Ok(tokio::task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&password_hash)?;
        Ok::<_, argon2::password_hash::Error>(
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok(),
        )
    })
    .await??)
}
