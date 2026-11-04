//! Deklarasi endpoint API inventaris.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/routes.md)
use axum::{
    Extension, Router, middleware,
    routing::{get, post},
};
use sqlx::PgPool;
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::{auth, handlers};

/// Menghubungkan handler API, pool database, session store, dan verifikator JWT.
pub fn create_router(pool: PgPool, session_cookie_secure: bool, jwt_secret: String) -> Router {
    let jwt = auth::jwt::JwtService::new(jwt_secret);

    let protected_routes = Router::new()
        .route("/api/auth/session", get(handlers::current_user))
        .route("/api/auth/logout", post(handlers::logout))
        .route(
            "/api/laptops",
            get(handlers::list_laptops).post(handlers::create_laptop),
        )
        .route(
            "/api/laptops/{id}",
            get(handlers::get_laptop)
                .patch(handlers::update_laptop)
                .delete(handlers::delete_laptop),
        )
        .route("/api/admin/users", post(handlers::create_user))
        .route(
            "/api/admin/users/{username}/role",
            axum::routing::patch(handlers::update_user_role),
        )
        // Memeriksa session atau JWT sebelum request diteruskan ke handler.
        .route_layer(middleware::from_fn_with_state(
            pool.clone(),
            auth::require_auth,
        ));

    Router::new()
        .route("/health", get(handlers::health))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/register", post(handlers::register))
        .merge(protected_routes)
        .layer(Extension(jwt))
        // Session disimpan di memori; cookie hanya `Secure` jika konfigurasi bernilai true.
        .layer(SessionManagerLayer::new(MemoryStore::default()).with_secure(session_cookie_secure))
        .with_state(pool)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use sqlx::postgres::PgPoolOptions;
    use tower::ServiceExt;

    use super::create_router;
    #[tokio::test]
    async fn protected_endpoints_require_a_session() {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://user:password@localhost/toko_laptop")
            .expect("lazy PostgreSQL pool should be valid");
        let app = create_router(
            pool,
            false,
            "test-secret-with-at-least-32-bytes-long".to_owned(),
        );

        let unauthorized = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/auth/session")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);

        let unauthenticated_laptops = app
            .oneshot(
                Request::builder()
                    .uri("/api/laptops")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(unauthenticated_laptops.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn protected_endpoints_reject_an_invalid_bearer_token() {
        let secret = "test-secret-with-at-least-32-bytes-long".to_owned();
        let token = "not-a-valid-token";
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://localhost/toko_laptop")
            .expect("lazy PostgreSQL pool should be valid");
        let app = create_router(pool, false, secret);

        let invalid_token = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/auth/session")
                    .header("Authorization", "Bearer invalid-token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(invalid_token.status(), StatusCode::UNAUTHORIZED);

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/auth/session")
                    .header("Authorization", format!("Bearer {token}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
