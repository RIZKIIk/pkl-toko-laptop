use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::PgPool;

use crate::{
    auth::middleware::AuthIdentity,
    error::ApiError,
    handlers::laptops::laptop_tidak_ditemukan,
    presenters::{laptop_response::laptop_ke_objek, purchase_response::struk_ke_objek},
    repositories::laptop_repository as repository,
    validation::validasi_id_laptop,
};

#[derive(Deserialize)]
pub struct BuyLaptopPayload {
    jumlah: i32,
    metode_pembayaran: String,
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
    if !matches!(
        payload.metode_pembayaran.as_str(),
        "qris" | "cash" | "bank_transfer" | "debit_card"
    ) {
        return Err(ApiError::BadRequest(
            "Metode pembayaran yang dipilih tidak tersedia".into(),
        ));
    }

    let (laptop, receipt) = match repository::beli_laptop(
        &pool,
        identity.user_id,
        &identity.username,
        id,
        payload.jumlah,
        &payload.metode_pembayaran,
    )
    .await?
    {
        Some(result) => result,
        None => match repository::ambil_laptop(&pool, id).await? {
            Some(laptop) if laptop.stok < payload.jumlah => {
                return Err(ApiError::Conflict("Stok laptop tidak mencukupi".into()));
            }
            Some(_) => {
                return Err(ApiError::BadRequest(
                    "Total pembelian melebihi batas yang dapat diproses".into(),
                ));
            }
            None => return Err(laptop_tidak_ditemukan(id)),
        },
    };

    Ok(Json(json!({
        "status": "success",
        "message": "Pembelian berhasil",
        "data": {
            "laptop": laptop_ke_objek(&laptop),
            "jumlah_dibeli": payload.jumlah,
            "struk": struk_ke_objek(&receipt)
        }
    })))
}
