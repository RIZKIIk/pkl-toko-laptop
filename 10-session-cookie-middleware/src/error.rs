use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub enum AppError {
    Database(sqlx::Error),
    NotFound,
    InvalidInput(&'static str),
    Unauthorized,
    Session,
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::Database(error) => {
                eprintln!("Kesalahan database: {error}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Kesalahan database")
            }
            Self::NotFound => (StatusCode::NOT_FOUND, "Laptop tidak ditemukan"),
            Self::InvalidInput(message) => (StatusCode::BAD_REQUEST, message),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Username atau password salah"),
            Self::Session => (StatusCode::INTERNAL_SERVER_ERROR, "Kesalahan session"),
        };
        (status, Json(json!({"status": "error", "message": message}))).into_response()
    }
}
