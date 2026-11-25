//! Handler untuk memperbarui sebagian field laptop.
//! Catatan: [Buka penjelasan](../../../catatan/rust/handlers/laptops/update.md)
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use serde_json::Value;
use sqlx::PgPool;

use crate::{
    auth::middleware::AuthIdentity,
    error::ApiError,
    handlers::laptops::{laptop_tidak_ditemukan, respons_berhasil_dengan_pesan},
    models::UpdateLaptopPayload,
    presenters::laptop_response::laptop_ke_array,
    repositories::laptop_repository as repository,
    validation::{validasi_id_laptop, validasi_perubahan_laptop},
};

/// Memvalidasi dan menyimpan perubahan parsial pada laptop.
pub async fn ubah_laptop(
    State(pool): State<PgPool>,
    Extension(identity): Extension<AuthIdentity>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateLaptopPayload>,
) -> Result<Json<Value>, ApiError> {
    validasi_id_laptop(id)?;
    let payload = validasi_perubahan_laptop(payload)?;

    let laptop = repository::ubah_laptop(&pool, identity.user_id, id, &payload)
        .await?
        .ok_or_else(|| laptop_tidak_ditemukan(id))?;

    Ok(Json(respons_berhasil_dengan_pesan(
        "Laptop berhasil diperbarui",
        laptop_ke_array(&laptop),
    )))
}
