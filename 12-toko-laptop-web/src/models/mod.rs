//! Model dan payload request yang digunakan bersama oleh lapisan backend.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/models/mod.md)
pub mod laptop;

pub use laptop::{CreateLaptopPayload, Laptop, UpdateLaptopPayload};
