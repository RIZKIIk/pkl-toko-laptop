use axum::{Router, routing::get};
use sqlx::PgPool;
use tower_sessions::{MemoryStore, SessionManagerLayer};

pub fn create_router(pool: PgPool, secure_cookie: bool) -> Router {
    Router::new()
        .route("/health", get(crate::handlers::laptops::health))
        .route(
            "/api/auth/login",
            axum::routing::post(crate::handlers::auth::login),
        )
        .route(
            "/api/auth/session",
            get(crate::handlers::auth::current_user),
        )
        .route(
            "/api/auth/logout",
            axum::routing::post(crate::handlers::auth::logout),
        )
        .route(
            "/api/laptops",
            get(crate::handlers::laptops::list).post(crate::handlers::laptops::create),
        )
        .route(
            "/api/laptops/{id}",
            get(crate::handlers::laptops::get)
                .put(crate::handlers::laptops::update)
                .delete(crate::handlers::laptops::delete),
        )
        .layer(SessionManagerLayer::new(MemoryStore::default()).with_secure(secure_cookie))
        .with_state(pool)
}
