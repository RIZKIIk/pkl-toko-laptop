//! Handler operasi laptop, dengan satu file untuk setiap aksi HTTP.
//! Dokumentasi: file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/handlers/laptops/mod.md
use crate::error::ApiError;
use serde_json::{Value, json};

mod create;
mod delete;
mod read;
mod update;

pub use create::create_laptop;
pub use delete::delete_laptop;
pub use read::{get_laptop, list_laptops};
pub use update::update_laptop;

/// Membentuk response sukses yang membawa data.
pub(super) fn success_response(data: Value) -> Value {
    json!({"status": "success", "data": data})
}

/// Membentuk response sukses dengan pesan dan data.
pub(super) fn success_response_with_message(message: &str, data: Value) -> Value {
    json!({
        "status": "success",
        "message": message,
        "data": data
    })
}

/// Membuat error 404 dengan pesan laptop yang konsisten.
pub(super) fn not_found_laptop(id: i64) -> ApiError {
    ApiError::NotFound(format!("Laptop dengan ID {id} tidak ditemukan"))
}
