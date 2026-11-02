//! Body JSON dengan field opsional untuk mengubah sebagian data laptop.
//! Dokumentasi: file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/models/laptop/update-payload.md
use serde::Deserialize;

/// Data parsial yang diterima oleh `PATCH /api/laptops/{id}`.
#[derive(Debug, Deserialize)]
pub struct UpdateLaptopPayload {
    /// Nama baru; `None` berarti nilai lama dipertahankan.
    pub nama: Option<String>,
    /// Merek baru; `None` berarti nilai lama dipertahankan.
    pub merek: Option<String>,
    /// Harga baru; `None` berarti nilai lama dipertahankan.
    pub harga: Option<i64>,
    /// Stok baru; `None` berarti nilai lama dipertahankan.
    pub stok: Option<i32>,
}
