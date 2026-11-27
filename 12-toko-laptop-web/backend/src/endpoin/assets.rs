//! Endpoint aset frontend yang dibatasi pada daftar file yang dikenal.
use axum::{
    Router,
    extract::Path,
    http::{HeaderValue, StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
    routing::get,
};
use sqlx::PgPool;

pub(super) fn router() -> Router<PgPool> {
    Router::<PgPool>::new().route("/assets/{*path}", get(aset_frontend))
}

async fn aset_frontend(Path(path): Path<String>) -> Response {
    let (content_type, contents) = match path.as_str() {
        "css/base.css" => (
            "text/css; charset=utf-8",
            include_str!("../../../frontend/css/base.css"),
        ),
        "css/login.css" => (
            "text/css; charset=utf-8",
            include_str!("../../../frontend/css/login.css"),
        ),
        "css/dashboard.css" => (
            "text/css; charset=utf-8",
            include_str!("../../../frontend/css/dashboard.css"),
        ),
        "css/components.css" => (
            "text/css; charset=utf-8",
            include_str!("../../../frontend/css/components.css"),
        ),
        "css/responsive.css" => (
            "text/css; charset=utf-8",
            include_str!("../../../frontend/css/responsive.css"),
        ),
        "css/theme.css" => (
            "text/css; charset=utf-8",
            include_str!("../../../frontend/css/theme.css"),
        ),
        "images/laptopku-hero.svg" => (
            "image/svg+xml; charset=utf-8",
            include_str!("../../../frontend/images/laptopku-hero.svg"),
        ),
        "js/app.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../../frontend/js/app.js"),
        ),
        "js/theme-init.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../../frontend/js/theme-init.js"),
        ),
        "js/shared.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../../frontend/js/shared.js"),
        ),
        "js/login.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../../frontend/js/login.js"),
        ),
        "js/theme.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../../frontend/js/theme.js"),
        ),
        "js/register.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../../frontend/js/register.js"),
        ),
        "js/sidebar.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../../frontend/js/sidebar.js"),
        ),
        "js/dashboard-view.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../../frontend/js/dashboard-view.js"),
        ),
        "js/dashboard.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../../frontend/js/dashboard.js"),
        ),
        "js/laptop-actions.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../../frontend/js/laptop-actions.js"),
        ),
        "js/user-actions.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../../../frontend/js/user-actions.js"),
        ),
        _ => return StatusCode::NOT_FOUND.into_response(),
    };

    (
        [(CONTENT_TYPE, HeaderValue::from_static(content_type))],
        contents,
    )
        .into_response()
}
