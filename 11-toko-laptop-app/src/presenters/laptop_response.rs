//! Mengubah model laptop menjadi array atau object JSON response.
//! Dokumentasi: file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/presenters/laptop-response.md
use crate::models::Laptop;
use serde_json::{Value, json};

use super::rupiah::formatkan_rupiah;

/// Membentuk array dengan urutan `[id, nama, merek, harga, stok]`.
pub(crate) fn laptop_to_array(laptop: &Laptop) -> Value {
    json!([
        laptop.id,
        laptop.nama,
        laptop.merek,
        formatkan_rupiah(laptop.harga),
        laptop.stok
    ])
}

/// Membentuk object bernama untuk response laptop yang baru dibuat.
pub(crate) fn laptop_to_object(laptop: &Laptop) -> Value {
    json!({
        "id": laptop.id,
        "nama": laptop.nama,
        "merek": laptop.merek,
        "harga": formatkan_rupiah(laptop.harga),
        "stok": laptop.stok
    })
}

#[cfg(test)]
mod tests {
    use super::{laptop_to_array, laptop_to_object};
    use crate::models::Laptop;
    use serde_json::json;

    fn sample_laptop() -> Laptop {
        Laptop {
            id: 12,
            nama: "ThinkPad E14".into(),
            merek: "Lenovo".into(),
            harga: 12_500_000,
            stok: 5,
        }
    }

    #[test]
    fn array_response_uses_the_documented_field_order() {
        assert_eq!(
            laptop_to_array(&sample_laptop()),
            json!([12, "ThinkPad E14", "Lenovo", "Rp 12.500.000", 5])
        );
    }

    #[test]
    fn object_response_keeps_harga_field_and_formats_it() {
        assert_eq!(
            laptop_to_object(&sample_laptop()),
            json!({
                "id": 12,
                "nama": "ThinkPad E14",
                "merek": "Lenovo",
                "harga": "Rp 12.500.000",
                "stok": 5
            })
        );
    }
}
