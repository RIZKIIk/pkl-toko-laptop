//! Halaman frontend dan redirect root.
use axum::{Router, response::Redirect, routing::get};
use sqlx::PgPool;

pub(super) fn router() -> Router<PgPool> {
    Router::<PgPool>::new()
        .route("/", get(ke_login))
        .route("/login", get(halaman_frontend))
        .route("/register", get(halaman_frontend))
        .route("/dashboard", get(halaman_frontend))
        .route("/dashboard/profile", get(halaman_frontend))
        .route("/dashboard/users/new", get(halaman_frontend))
}

async fn ke_login() -> Redirect {
    Redirect::temporary("/login")
}

async fn halaman_frontend() -> axum::response::Html<&'static str> {
    axum::response::Html(include_str!("../../../frontend/index.html"))
}
