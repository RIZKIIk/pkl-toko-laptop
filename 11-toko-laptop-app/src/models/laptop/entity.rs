//! Bentuk satu baris laptop yang dipetakan dari PostgreSQL.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/models/laptop/entity.md)
use sqlx::FromRow;

/// Data inventaris laptop yang dipakai repository dan presenter.
#[derive(Debug, FromRow)]
pub struct Laptop {
    /// ID unik yang dibuat oleh database.
    pub id: i64,
    /// Nama atau model laptop.
    pub nama: String,
    /// Merek pembuat laptop.
    pub merek: String,
    /// Harga dalam rupiah utuh.
    pub harga: i64,
    /// Jumlah unit yang tersedia.
    pub stok: i32,
}
