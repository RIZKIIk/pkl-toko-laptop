use axum::{Router, middleware, routing::get};
use sqlx::PgPool;
use tower_sessions::{MemoryStore, SessionManagerLayer};

pub fn create_router(pool: PgPool, secure_cookie: bool) -> Router {
    let protected_routes: Router<PgPool> = Router::new()
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
        .route_layer(middleware::from_fn(crate::middleware::require_auth));

    Router::new()
        .route("/health", get(crate::handlers::laptops::health))
        .route(
            "/api/auth/login",
            axum::routing::post(crate::handlers::auth::login),
        )
        .merge(protected_routes)
        .layer(SessionManagerLayer::new(MemoryStore::default()).with_secure(secure_cookie))
        .with_state(pool)
}
