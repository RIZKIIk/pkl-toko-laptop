//! Endpoint sesi yang hanya tersedia setelah autentikasi.
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::handlers::auth;

pub(super) fn router() -> Router<PgPool> {
    Router::<PgPool>::new()
        .route("/api/auth/session", get(auth::pengguna_saat_ini))
        .route("/api/auth/logout", post(auth::keluar))
}
