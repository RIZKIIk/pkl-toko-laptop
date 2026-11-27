//! Body JSON dengan semua field wajib untuk membuat laptop.
//! Catatan: [Buka penjelasan](../../../catatan/rust/models/laptop/create-payload.md)
use serde::Deserialize;

/// Data yang diterima oleh `POST /api/laptops`.
#[derive(Debug, Deserialize)]
pub struct CreateLaptopPayload {
    /// Nama atau model laptop.
    pub nama: String,
    /// Merek pembuat laptop.
    pub merek: String,
    /// Deskripsi laptop yang ditampilkan di katalog.
    pub deskripsi: String,
    /// Harga dalam rupiah utuh.
    pub harga: i64,
    /// Jumlah stok awal.
    pub stok: i32,
}
