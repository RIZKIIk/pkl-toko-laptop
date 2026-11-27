//! Validasi payload lengkap untuk pembuatan laptop.
//! Catatan: [Buka penjelasan](../../catatan/rust/validation/create.md)
use crate::{error::ApiError, models::CreateLaptopPayload};

use super::text::validasi_teks;

/// Membersihkan teks dan menolak harga atau stok negatif.
pub fn validasi_laptop_baru(payload: CreateLaptopPayload) -> Result<CreateLaptopPayload, ApiError> {
    let nama = validasi_teks(payload.nama, "Nama laptop", 150)?;
    let merek = validasi_teks(payload.merek, "Merek laptop", 100)?;
    let deskripsi = validasi_teks(payload.deskripsi, "Deskripsi laptop", 1000)?;

    if payload.harga < 0 {
        return Err(ApiError::BadRequest("Harga tidak boleh negatif".into()));
    }
    if payload.stok < 0 {
        return Err(ApiError::BadRequest("Stok tidak boleh negatif".into()));
    }

    Ok(CreateLaptopPayload {
        nama,
        merek,
        deskripsi,
        harga: payload.harga,
        stok: payload.stok,
    })
}
