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
    handlers::laptops::{laptop_tidak_ditemukan, respons_berhasil},
    presenters::laptop_response::laptop_ke_array,
    repositories::laptop_repository as repository,
    validation::validasi_id_laptop,
};

/// Mengambil seluruh laptop dan mengubahnya menjadi response JSON.
pub async fn daftar_laptop(State(pool): State<PgPool>) -> Result<Json<Value>, ApiError> {
    let laptops = repository::daftar_laptop(&pool).await?;
    let data = laptops.iter().map(laptop_ke_array).collect::<Vec<_>>();

    Ok(Json(respons_berhasil(json!(data))))
}

/// Mengambil satu laptop berdasarkan ID.
pub async fn ambil_laptop(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    validasi_id_laptop(id)?;

    let laptop = repository::ambil_laptop(&pool, id)
        .await?
        .ok_or_else(|| laptop_tidak_ditemukan(id))?;

    Ok(Json(respons_berhasil(laptop_ke_array(&laptop))))
}
