//! Handler untuk membaca daftar atau satu laptop.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/handlers/laptops/read.md)
use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::{Value, json};
use sqlx::PgPool;

use crate::{
    error::ApiError,
    handlers::laptops::{not_found_laptop, success_response},
    presenters::laptop_response::laptop_to_array,
    repositories::laptop_repository as repository,
    validation::validate_laptop_id,
};

/// Mengambil seluruh laptop dan mengubahnya menjadi response JSON.
pub async fn list_laptops(State(pool): State<PgPool>) -> Result<Json<Value>, ApiError> {
    let laptops = repository::list_laptops(&pool).await?;
    let data = laptops.iter().map(laptop_to_array).collect::<Vec<_>>();

    Ok(Json(success_response(json!(data))))
}

/// Mengambil satu laptop berdasarkan ID.
pub async fn get_laptop(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    validate_laptop_id(id)?;

    let laptop = repository::get_laptop(&pool, id)
        .await?
        .ok_or_else(|| not_found_laptop(id))?;

    Ok(Json(success_response(laptop_to_array(&laptop))))
}
