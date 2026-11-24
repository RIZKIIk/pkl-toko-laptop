//! Handler untuk menghapus laptop berdasarkan ID.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/handlers/laptops/delete.md)
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use serde_json::{Value, json};
use sqlx::PgPool;

use crate::{
    auth::middleware::AuthIdentity, error::ApiError, handlers::laptops::laptop_tidak_ditemukan,
    repositories::laptop_repository as repository, validation::validasi_id_laptop,
};

/// Menghapus laptop dan mengembalikan 404 jika ID tidak ditemukan.
pub async fn hapus_laptop(
    State(pool): State<PgPool>,
    Extension(identity): Extension<AuthIdentity>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    validasi_id_laptop(id)?;

    if !repository::hapus_laptop(&pool, identity.user_id, id).await? {
        return Err(laptop_tidak_ditemukan(id));
    }

    Ok(Json(
        json!({"status": "success", "message": "Laptop berhasil dihapus", "id": id}),
    ))
}
