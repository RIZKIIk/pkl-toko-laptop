//! Handler operasi laptop, dengan satu file untuk setiap aksi HTTP.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/handlers/laptops/mod.md)
use crate::error::ApiError;
use serde_json::{Value, json};

mod buy;
mod create;
mod delete;
mod read;
mod update;

pub use buy::beli_laptop;
pub use create::tambah_laptop;
pub use delete::hapus_laptop;
pub use read::{ambil_laptop, daftar_laptop};
pub use update::ubah_laptop;

/// Membentuk response sukses yang membawa data.
pub fn respons_berhasil(data: Value) -> Value {
    json!({"status": "success", "data": data})
}

/// Membentuk response sukses dengan pesan dan data.
pub fn respons_berhasil_dengan_pesan(message: &str, data: Value) -> Value {
    json!({
        "status": "success",
        "message": message,
        "data": data
    })
}

/// Membuat error 404 dengan pesan laptop yang konsisten.
pub fn laptop_tidak_ditemukan(id: i64) -> ApiError {
    ApiError::NotFound(format!("Laptop dengan ID {id} tidak ditemukan"))
}
