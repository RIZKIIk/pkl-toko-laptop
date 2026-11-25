//! Normalisasi dan validasi umum untuk field teks.
//! Catatan: [Buka penjelasan](../../catatan/rust/validation/text.md)
use crate::error::ApiError;

/// Menghapus spasi pinggir lalu memeriksa teks kosong dan batas karakter.
pub fn validasi_teks(value: String, field: &str, max_length: usize) -> Result<String, ApiError> {
    let value = value.trim().to_string();

    if value.is_empty() {
        return Err(ApiError::BadRequest(format!("{field} tidak boleh kosong")));
    }
    // Hitung karakter Unicode, bukan byte UTF-8, agar batas sesuai yang dilihat pengguna.
    if value.chars().count() > max_length {
        return Err(ApiError::BadRequest(format!(
            "{field} maksimal {max_length} karakter"
        )));
    }

    Ok(value)
}
