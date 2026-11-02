//! Validasi ID laptop yang diterima dari parameter path URL.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/validation/id.md)
use crate::error::ApiError;

/// Menolak ID nol atau negatif sebelum query database dijalankan.
pub fn validate_laptop_id(id: i64) -> Result<(), ApiError> {
    if id <= 0 {
        return Err(ApiError::BadRequest("ID harus berupa angka positif".into()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_laptop_id;

    #[test]
    fn laptop_id_must_be_positive() {
        assert!(validate_laptop_id(1).is_ok());
        assert!(validate_laptop_id(0).is_err());
        assert!(validate_laptop_id(-1).is_err());
    }
}
