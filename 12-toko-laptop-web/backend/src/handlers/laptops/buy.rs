use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::PgPool;

use crate::{
    auth::middleware::AuthIdentity, error::ApiError, handlers::laptops::laptop_tidak_ditemukan,
    presenters::laptop_response::laptop_ke_objek, repositories::laptop_repository as repository,
    validation::validasi_id_laptop,
};

#[derive(Deserialize)]
pub struct BuyLaptopPayload {
    jumlah: i32,
}

/// Membeli laptop dan mengurangi stok yang tersedia.
pub async fn beli_laptop(
    State(pool): State<PgPool>,
    axum::Extension(identity): axum::Extension<AuthIdentity>,
    Path(id): Path<i64>,
    Json(payload): Json<BuyLaptopPayload>,
) -> Result<Json<Value>, ApiError> {
    validasi_id_laptop(id)?;
    if payload.jumlah <= 0 {
        return Err(ApiError::BadRequest(
            "Jumlah pembelian harus lebih dari 0".into(),
        ));
    }

    let laptop = match repository::beli_laptop(&pool, identity.user_id, id, payload.jumlah).await? {
        Some(laptop) => laptop,
        None => match repository::ambil_laptop(&pool, id).await? {
            Some(_) => return Err(ApiError::Conflict("Stok laptop tidak mencukupi".into())),
            None => return Err(laptop_tidak_ditemukan(id)),
        },
    };

    Ok(Json(json!({
        "status": "success",
        "message": "Pembelian berhasil",
        "data": {
            "laptop": laptop_ke_objek(&laptop),
            "jumlah_dibeli": payload.jumlah
        }
    })))
}
