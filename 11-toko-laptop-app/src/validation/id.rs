//! Validasi ID laptop yang diterima dari parameter path URL.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/validation/id.md)
use crate::error::ApiError;

/// Menolak ID nol atau negatif sebelum query database dijalankan.
pub fn validasi_id_laptop(id: i64) -> Result<(), ApiError> {
    if id <= 0 {
        return Err(ApiError::BadRequest("ID harus berupa angka positif".into()));
    }

    Ok(())
}
