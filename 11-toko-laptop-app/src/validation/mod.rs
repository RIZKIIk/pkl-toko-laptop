//! Validasi ID dan payload laptop sebelum data masuk ke repository.
//! Dokumentasi: file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/validation/mod.md
mod create;
mod id;
mod text;
mod update;

pub use create::validate_create_laptop;
pub use id::validate_laptop_id;
pub use update::validate_update_laptop;
