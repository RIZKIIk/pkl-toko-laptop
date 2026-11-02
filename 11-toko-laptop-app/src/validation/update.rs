//! Validasi payload parsial untuk perubahan laptop.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/validation/update.md)
use crate::{error::ApiError, models::UpdateLaptopPayload};

use super::text::validate_text;

/// Memeriksa hanya field yang dikirim dan menolak payload tanpa perubahan.
pub fn validate_update_laptop(
    mut payload: UpdateLaptopPayload,
) -> Result<UpdateLaptopPayload, ApiError> {
    // `take()` memungkinkan teks dinormalisasi lalu dikembalikan ke field opsional.
    if let Some(nama) = payload.nama.take() {
        payload.nama = Some(validate_text(nama, "Nama laptop", 150)?);
    }
    if let Some(merek) = payload.merek.take() {
        payload.merek = Some(validate_text(merek, "Merek laptop", 100)?);
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

#[cfg(test)]
mod tests {
    use super::validate_update_laptop;
    use crate::models::UpdateLaptopPayload;

    #[test]
    fn update_accepts_a_single_field_and_normalizes_text() {
        let payload = UpdateLaptopPayload {
            nama: Some(" ThinkPad ".into()),
            merek: None,
            harga: None,
            stok: Some(4),
        };
        let validated =
            validate_update_laptop(payload).unwrap_or_else(|_| panic!("payload should be valid"));

        assert_eq!(validated.nama.as_deref(), Some("ThinkPad"));
        assert_eq!(validated.stok, Some(4));
    }

    #[test]
    fn update_rejects_an_empty_payload() {
        let payload = UpdateLaptopPayload {
            nama: None,
            merek: None,
            harga: None,
            stok: None,
        };

        assert!(validate_update_laptop(payload).is_err());
    }
}
