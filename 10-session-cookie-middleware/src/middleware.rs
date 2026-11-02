use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use tower_sessions::Session;

pub async fn require_auth(
    session: Session,
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    match session.get::<String>("username").await {
        Ok(Some(_)) => Ok(next.run(request).await),
        Ok(None) => Err((StatusCode::UNAUTHORIZED, "Belum login").into_response()),
        Err(error) => {
            eprintln!("Gagal memeriksa session: {error}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Kesalahan session").into_response())
        }
    }
}
