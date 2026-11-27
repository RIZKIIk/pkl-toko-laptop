//! Komposisi router; definisi endpoint berada di modul berdasarkan fiturnya.
//! Catatan: [Buka penjelasan](../../catatan/rust/routes.md)
mod admin;
mod assets;
mod health;
mod laptops;
mod login;
mod pages;
mod register;
mod session;

use axum::{Extension, Router, middleware};
use sqlx::PgPool;
use tower_sessions::SessionManagerLayer;
use tower_sessions_redis_store::{RedisStore, fred::prelude::Pool};

use crate::auth::{self, token_store::JwtTokenStore};

/// Menggabungkan route fitur dan memasang middleware serta state aplikasi.
pub fn buat_router(
    pool: PgPool,
    session_cookie_secure: bool,
    jwt_secret: String,
    redis_pool: Pool,
) -> Router {
    let jwt = auth::jwt::JwtService::buat(jwt_secret);
    let jwt_token_store = JwtTokenStore::baru(redis_pool.clone());
    let session_store = RedisStore::new(redis_pool);

    let protected_routes = Router::<PgPool>::new()
        .merge(session::router())
        .merge(laptops::router())
        .merge(admin::router())
        .route_layer(middleware::from_fn_with_state(
            pool.clone(),
            auth::wajibkan_autentikasi,
        ));

    Router::<PgPool>::new()
        .merge(pages::router())
        .merge(assets::router())
        .merge(health::router())
        .merge(login::router())
        .merge(register::router())
        .merge(protected_routes)
        .layer(Extension(jwt))
        .layer(Extension(jwt_token_store))
        .layer(SessionManagerLayer::new(session_store).with_secure(session_cookie_secure))
        .with_state(pool)
}
