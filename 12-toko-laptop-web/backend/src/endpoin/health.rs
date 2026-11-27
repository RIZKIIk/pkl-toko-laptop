//! Endpoint status untuk memeriksa apakah server merespons.
use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::system;

pub(super) fn router() -> Router<PgPool> {
    Router::<PgPool>::new().route("/health", get(system::cek_kesehatan))
}
