//! Mengubah model laptop menjadi array atau object JSON response.
//! Catatan: [Buka penjelasan](../../catatan/rust/presenters/laptop-response.md)
use crate::models::Laptop;
use serde_json::{Value, json};

use super::rupiah::formatkan_rupiah;

/// Membentuk array dengan urutan `[id, nama, merek, harga, stok, deskripsi]`.
pub fn laptop_ke_array(laptop: &Laptop) -> Value {
    json!([
        laptop.id,
        laptop.nama,
        laptop.merek,
        formatkan_rupiah(laptop.harga),
        laptop.stok,
        laptop.deskripsi
    ])
}

/// Membentuk object bernama untuk response laptop yang baru dibuat.
pub fn laptop_ke_objek(laptop: &Laptop) -> Value {
    json!({
        "id": laptop.id,
        "nama": laptop.nama,
        "merek": laptop.merek,
        "deskripsi": laptop.deskripsi,
        "harga": formatkan_rupiah(laptop.harga),
        "stok": laptop.stok
    })
}
