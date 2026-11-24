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
pub fn buat_router(pool: PgPool, session_cookie_secure: bool, jwt_secret: String) -> Router {
    let jwt = auth::jwt::JwtService::buat(jwt_secret);

    let protected_routes = Router::new()
        .route("/api/auth/session", get(handlers::pengguna_saat_ini))
        .route("/api/auth/logout", post(handlers::keluar))
        .route(
            "/api/laptops",
            get(handlers::daftar_laptop).post(handlers::tambah_laptop),
        )
        .route(
            "/api/laptops/{id}",
            get(handlers::ambil_laptop)
                .patch(handlers::ubah_laptop)
                .delete(handlers::hapus_laptop),
        )
        .route("/api/laptops/{id}/buy", post(handlers::beli_laptop))
        .route("/api/admin/users", post(handlers::buat_pengguna))
        .route(
            "/api/admin/users/{username}/role",
            axum::routing::patch(handlers::ubah_peran_pengguna),
        )
        // Memeriksa session atau JWT sebelum request diteruskan ke handler.
        .route_layer(middleware::from_fn_with_state(
            pool.clone(),
            auth::wajibkan_autentikasi,
        ));

    Router::new()
        .route("/health", get(handlers::cek_kesehatan))
        .route("/api/auth/login", post(handlers::masuk))
        .route("/api/auth/register", post(handlers::daftar))
        .merge(protected_routes)
        .layer(Extension(jwt))
        // Session disimpan di memori; cookie hanya `Secure` jika konfigurasi bernilai true.
        .layer(SessionManagerLayer::new(MemoryStore::default()).with_secure(session_cookie_secure))
        .with_state(pool)
}
