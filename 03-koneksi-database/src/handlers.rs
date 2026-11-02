use axum::{Json, extract::State, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;

pub async fn health(State(pool): State<PgPool>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|error| {
            eprintln!("Pemeriksaan koneksi database gagal: {error}");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"status": "error", "message": "Database tidak tersedia"})),
            )
        })?;

    Ok(Json(json!({"status": "ok", "database": "connected"})))
}
