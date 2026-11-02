use crate::{error::AppError, models::LoginPayload, repository};
use axum::{Json, extract::State, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
use tower_sessions::Session;

pub async fn login(
    State(pool): State<PgPool>,
    session: Session,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>, AppError> {
    if payload.username.trim().is_empty() || payload.password.is_empty() {
        return Err(AppError::InvalidInput("Username dan password wajib diisi."));
    }
    let Some(username) = repository::authenticate(&pool, &payload).await? else {
        return Err(AppError::Unauthorized);
    };
    session.cycle_id().await.map_err(|error| {
        eprintln!("Gagal membuat session: {error}");
        AppError::Session
    })?;
    session
        .insert("username", username)
        .await
        .map_err(|error| {
            eprintln!("Gagal menyimpan session: {error}");
            AppError::Session
        })?;
    Ok(Json(
        json!({"status": "success", "message": "Login berhasil"}),
    ))
}

pub async fn current_user(session: Session) -> Result<Json<Value>, AppError> {
    match session.get::<String>("username").await {
        Ok(Some(username)) => Ok(Json(json!({"status": "success", "username": username}))),
        Ok(None) => Err(AppError::Unauthorized),
        Err(error) => {
            eprintln!("Gagal membaca session: {error}");
            Err(AppError::Session)
        }
    }
}

pub async fn logout(session: Session) -> Result<(StatusCode, Json<Value>), AppError> {
    session.flush().await.map_err(|error| {
        eprintln!("Gagal menghapus session: {error}");
        AppError::Session
    })?;
    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "message": "Logout berhasil"})),
    ))
}
