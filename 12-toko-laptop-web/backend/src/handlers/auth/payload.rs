//! Bentuk body JSON untuk endpoint autentikasi dan akun.
use serde::Deserialize;

/// Kredensial login dan registrasi publik.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CredentialsPayload {
    pub(super) username: String,
    pub(super) password: String,
}

/// Payload endpoint login.
pub type LoginPayload = CredentialsPayload;

/// Payload endpoint registrasi.
pub type RegisterPayload = CredentialsPayload;

/// Body JSON untuk membuat akun buyer atau restocker oleh admin.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateUserPayload {
    pub(super) username: String,
    pub(super) password: String,
    pub(super) role: crate::auth::role::UserRole,
}

/// Body JSON untuk mengubah role akun.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateUserRolePayload {
    pub(super) role: crate::auth::role::UserRole,
}
