//! Handler untuk membuat laptop baru.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/handlers/laptops/create.md)
use axum::{Json, extract::State, http::StatusCode};
use serde_json::Value;
use sqlx::PgPool;

use crate::{
    error::ApiError, handlers::laptops::respons_berhasil_dengan_pesan, models::CreateLaptopPayload,
    presenters::laptop_response::laptop_ke_objek, repositories::laptop_repository as repository,
    validation::validasi_laptop_baru,
};

/// Memvalidasi request, menyimpan laptop, lalu mengembalikan status 201.
pub async fn tambah_laptop(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateLaptopPayload>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let payload = validasi_laptop_baru(payload)?;
    let laptop = repository::tambah_laptop(&pool, &payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(respons_berhasil_dengan_pesan(
            "Laptop berhasil ditambahkan",
            laptop_ke_objek(&laptop),
        )),
    ))
}
