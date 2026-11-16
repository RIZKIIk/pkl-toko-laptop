//! Validasi payload parsial untuk perubahan laptop.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/validation/update.md)
use crate::{error::ApiError, models::UpdateLaptopPayload};

use super::text::validasi_teks;

/// Memeriksa hanya field yang dikirim dan menolak payload tanpa perubahan.
pub fn validasi_perubahan_laptop(
    mut payload: UpdateLaptopPayload,
) -> Result<UpdateLaptopPayload, ApiError> {
    // `take()` memungkinkan teks dinormalisasi lalu dikembalikan ke field opsional.
    if let Some(nama) = payload.nama.take() {
        payload.nama = Some(validasi_teks(nama, "Nama laptop", 150)?);
    }
    if let Some(merek) = payload.merek.take() {
        payload.merek = Some(validasi_teks(merek, "Merek laptop", 100)?);
    }
    if payload.harga.is_some_and(|harga| harga < 0) {
        return Err(ApiError::BadRequest("Harga tidak boleh negatif".into()));
    }
    if payload.stok.is_some_and(|stok| stok < 0) {
        return Err(ApiError::BadRequest("Stok tidak boleh negatif".into()));
    }
    if payload.nama.is_none()
        && payload.merek.is_none()
        && payload.harga.is_none()
        && payload.stok.is_none()
    {
        return Err(ApiError::BadRequest(
            "Minimal satu field harus dikirim untuk diperbarui".into(),
        ));
    }

    Ok(payload)
}
