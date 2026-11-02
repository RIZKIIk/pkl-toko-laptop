use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(crate::handlers::health))
        .route(
            "/api/laptops",
            get(crate::handlers::list).post(crate::handlers::create),
        )
        .route(
            "/api/laptops/{id}",
            get(crate::handlers::get)
                .put(crate::handlers::update)
                .delete(crate::handlers::delete),
        )
        .with_state(pool)
}
