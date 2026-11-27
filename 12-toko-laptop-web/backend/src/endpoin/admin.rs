//! Endpoint administrasi akun dan daftar role.
use axum::{
    Router,
    routing::{get, patch, post},
};
use sqlx::PgPool;

use crate::handlers::auth;

pub(super) fn router() -> Router<PgPool> {
    Router::<PgPool>::new()
        .route("/api/admin/roles", get(auth::daftar_peran_admin))
        .route("/api/admin/users", post(auth::buat_pengguna))
        .route(
            "/api/admin/users/{username}/role",
            patch(auth::ubah_peran_pengguna),
        )
}
