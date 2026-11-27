//! Deklarasi endpoint API inventaris.
//! Catatan: [Buka penjelasan](../catatan/rust/routes.md)
use axum::{
    Extension, Router,
    extract::Path,
    http::{HeaderValue, StatusCode, header::CONTENT_TYPE},
    middleware,
    response::Redirect,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use sqlx::PgPool;
use tower_sessions::SessionManagerLayer;
use tower_sessions_redis_store::{RedisStore, fred::prelude::Pool};

use crate::{auth, handlers};

/// Menghubungkan handler API, pool database, session store, dan verifikator JWT.
pub fn buat_router(
    pool: PgPool,
    session_cookie_secure: bool,
    jwt_secret: String,
    redis_pool: Pool,
) -> Router {
    let jwt = auth::jwt::JwtService::buat(jwt_secret);
    let session_store = RedisStore::new(redis_pool);

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
        .route("/api/admin/roles", get(handlers::daftar_peran_admin))
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
        .route("/", get(|| async { Redirect::temporary("/login") }))
        .route("/login", get(halaman_frontend))
        .route("/register", get(halaman_frontend))
        .route("/dashboard", get(halaman_frontend))
        .route("/assets/{*path}", get(aset_frontend))
        .route("/health", get(handlers::cek_kesehatan))
        .route("/api/auth/login", post(handlers::masuk))
        .route("/api/auth/register", post(handlers::daftar))
        .merge(protected_routes)
        .layer(Extension(jwt))
        // Redis menyimpan data session; middleware tetap mengelola cookie browser.
        .layer(SessionManagerLayer::new(session_store).with_secure(session_cookie_secure))
        .with_state(pool)
}

/// Menyajikan aplikasi web untuk halaman login dan dashboard.
async fn halaman_frontend() -> axum::response::Html<&'static str> {
    axum::response::Html(include_str!("../../frontend/index.html"))
}

/// Mengirim file frontend yang sudah di-allowlist saat compile time.
async fn aset_frontend(Path(path): Path<String>) -> Response {
    let (content_type, contents) = match path.as_str() {
        "css/base.css" => (
            "text/css; charset=utf-8",
            include_str!("../../frontend/css/base.css"),
        ),
        "css/login.css" => (
            "text/css; charset=utf-8",
            include_str!("../../frontend/css/login.css"),
        ),
        "css/dashboard.css" => (
            "text/css; charset=utf-8",
            include_str!("../../frontend/css/dashboard.css"),
        ),
        "css/components.css" => (
            "text/css; charset=utf-8",
            include_str!("../../frontend/css/components.css"),
        ),
        "css/responsive.css" => (
            "text/css; charset=utf-8",
            include_str!("../../frontend/css/responsive.css"),
        ),
        "css/theme.css" => (
            "text/css; charset=utf-8",
            include_str!("../../frontend/css/theme.css"),
        ),
        "images/laptopku-hero.svg" => (
            "image/svg+xml; charset=utf-8",
            include_str!("../../frontend/images/laptopku-hero.svg"),
        ),
        "js/app.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../frontend/js/app.js"),
        ),
        "js/theme-init.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../frontend/js/theme-init.js"),
        ),
        "js/shared.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../frontend/js/shared.js"),
        ),
        "js/login.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../frontend/js/login.js"),
        ),
        "js/theme.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../frontend/js/theme.js"),
        ),
        "js/register.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../frontend/js/register.js"),
        ),
        "js/sidebar.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../frontend/js/sidebar.js"),
        ),
        "js/dashboard-view.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../frontend/js/dashboard-view.js"),
        ),
        "js/dashboard.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../frontend/js/dashboard.js"),
        ),
        "js/laptop-actions.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../frontend/js/laptop-actions.js"),
        ),
        "js/user-actions.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../frontend/js/user-actions.js"),
        ),
        _ => return StatusCode::NOT_FOUND.into_response(),
    };

    (
        [(CONTENT_TYPE, HeaderValue::from_static(content_type))],
        contents,
    )
        .into_response()
}
