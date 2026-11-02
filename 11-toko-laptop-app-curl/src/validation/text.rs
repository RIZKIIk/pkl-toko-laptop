//! Normalisasi dan validasi umum untuk field teks.
//! Dokumentasi: file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/validation/text.md
use crate::error::ApiError;

/// Menghapus spasi pinggir lalu memeriksa teks kosong dan batas karakter.
pub(super) fn validate_text(
    value: String,
    field: &str,
    max_length: usize,
) -> Result<String, ApiError> {
    let value = value.trim().to_string();

    if value.is_empty() {
        return Err(ApiError::BadRequest(format!("{field} tidak boleh kosong")));
    }
    if value.chars().count() > max_length {
        return Err(ApiError::BadRequest(format!(
            "{field} maksimal {max_length} karakter"
        )));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::validate_text;

    #[test]
    fn trims_surrounding_whitespace() {
        assert!(matches!(
            validate_text("  ThinkPad  ".into(), "Nama laptop", 150),
            Ok(text) if text == "ThinkPad"
        ));
    }

    #[test]
    fn rejects_empty_text_after_trimming() {
        assert!(validate_text("   ".into(), "Nama laptop", 150).is_err());
    }

    #[test]
    fn counts_unicode_characters_instead_of_bytes() {
        assert!(matches!(
            validate_text("é".into(), "Nama laptop", 1),
            Ok(text) if text == "é"
        ));
        assert!(validate_text("éé".into(), "Nama laptop", 1).is_err());
    }
}
