//! Validasi payload lengkap untuk pembuatan laptop.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/validation/create.md)
use crate::{error::ApiError, models::CreateLaptopPayload};

use super::text::validate_text;

/// Membersihkan teks dan menolak harga atau stok negatif.
pub fn validate_create_laptop(
    payload: CreateLaptopPayload,
) -> Result<CreateLaptopPayload, ApiError> {
    let nama = validate_text(payload.nama, "Nama laptop", 150)?;
    let merek = validate_text(payload.merek, "Merek laptop", 100)?;

    if payload.harga < 0 {
        return Err(ApiError::BadRequest("Harga tidak boleh negatif".into()));
    }
    if payload.stok < 0 {
        return Err(ApiError::BadRequest("Stok tidak boleh negatif".into()));
    }

    Ok(CreateLaptopPayload {
        nama,
        merek,
        harga: payload.harga,
        stok: payload.stok,
    })
}

#[cfg(test)]
mod tests {
    use super::validate_create_laptop;
    use crate::models::CreateLaptopPayload;

    #[test]
    fn create_payload_trims_text_and_validates_fields() {
        let payload = CreateLaptopPayload {
            nama: " ThinkPad ".into(),
            merek: " Lenovo ".into(),
            harga: 12_500_000,
            stok: 5,
        };

        let validated = validate_create_laptop(payload)
            .unwrap_or_else(|_| panic!("valid create payload should pass"));

        assert_eq!(validated.nama, "ThinkPad");
        assert_eq!(validated.merek, "Lenovo");
    }

    #[test]
    fn create_rejects_negative_price() {
        let payload = CreateLaptopPayload {
            nama: "Laptop".into(),
            merek: "Merek".into(),
            harga: -1,
            stok: 0,
        };

        assert!(validate_create_laptop(payload).is_err());
    }
}
