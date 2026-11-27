//! Endpoint registrasi akun publik.
use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::handlers::auth;

pub(super) fn router() -> Router<PgPool> {
    Router::<PgPool>::new().route("/api/auth/register", post(auth::daftar))
}
