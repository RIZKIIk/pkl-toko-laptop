//! Body JSON dengan semua field wajib untuk membuat laptop.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/models/laptop/create-payload.md)
use serde::Deserialize;

/// Data yang diterima oleh `POST /api/laptops`.
#[derive(Debug, Deserialize)]
pub struct CreateLaptopPayload {
    /// Nama atau model laptop.
    pub nama: String,
    /// Merek pembuat laptop.
    pub merek: String,
    /// Harga dalam rupiah utuh.
    pub harga: i64,
    /// Jumlah stok awal.
    pub stok: i32,
}
