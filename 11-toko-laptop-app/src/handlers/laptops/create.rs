//! Handler untuk membuat laptop baru.
//! Dokumentasi: file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/handlers/laptops/create.md
use axum::{Json, extract::State, http::StatusCode};
use serde_json::Value;
use sqlx::PgPool;

use crate::{
    error::ApiError, handlers::laptops::success_response_with_message, models::CreateLaptopPayload,
    presenters::laptop_response::laptop_to_object, repositories::laptop_repository as repository,
    validation::validate_create_laptop,
};

/// Memvalidasi request, menyimpan laptop, lalu mengembalikan status 201.
pub async fn create_laptop(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateLaptopPayload>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let payload = validate_create_laptop(payload)?;
    let laptop = repository::create_laptop(&pool, &payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(success_response_with_message(
            "Laptop berhasil ditambahkan",
            laptop_to_object(&laptop),
        )),
    ))
}
