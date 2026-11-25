//! Validasi ID dan payload laptop sebelum data masuk ke repository.
//! Catatan: [Buka penjelasan](../../catatan/rust/validation/mod.md)
mod create;
mod id;
mod text;
mod update;

pub use create::validasi_laptop_baru;
pub use id::validasi_id_laptop;
pub use update::validasi_perubahan_laptop;
