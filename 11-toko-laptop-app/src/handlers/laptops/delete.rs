//! Handler untuk menghapus laptop berdasarkan ID.
//! Dokumentasi: file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/handlers/laptops/delete.md
use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::{Value, json};
use sqlx::PgPool;

use crate::{
    error::ApiError, handlers::laptops::not_found_laptop,
    repositories::laptop_repository as repository, validation::validate_laptop_id,
};

/// Menghapus laptop dan mengembalikan 404 jika ID tidak ditemukan.
pub async fn delete_laptop(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    validate_laptop_id(id)?;

    if !repository::delete_laptop(&pool, id).await? {
        return Err(not_found_laptop(id));
    }

    Ok(Json(
        json!({"status": "success", "message": "Laptop berhasil dihapus", "id": id}),
    ))
}
