use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(crate::handlers::health))
        .route("/api/laptops", get(crate::handlers::list_laptops))
        .with_state(pool)
}
