//! Handler untuk memperbarui sebagian field laptop.
//! Dokumentasi: file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/handlers/laptops/update.md
use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::Value;
use sqlx::PgPool;

use crate::{
    error::ApiError,
    handlers::laptops::{not_found_laptop, success_response_with_message},
    models::UpdateLaptopPayload,
    presenters::laptop_response::laptop_to_array,
    repositories::laptop_repository as repository,
    validation::{validate_laptop_id, validate_update_laptop},
};

/// Memvalidasi dan menyimpan perubahan parsial pada laptop.
pub async fn update_laptop(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateLaptopPayload>,
) -> Result<Json<Value>, ApiError> {
    validate_laptop_id(id)?;
    let payload = validate_update_laptop(payload)?;

    let laptop = repository::update_laptop(&pool, id, &payload)
        .await?
        .ok_or_else(|| not_found_laptop(id))?;

    Ok(Json(success_response_with_message(
        "Laptop berhasil diperbarui",
        laptop_to_array(&laptop),
    )))
}
