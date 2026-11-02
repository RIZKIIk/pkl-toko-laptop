//! Model dan payload request yang digunakan bersama oleh lapisan backend.
//! Dokumentasi: file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/models/mod.md
pub mod laptop;

pub use laptop::{CreateLaptopPayload, Laptop, UpdateLaptopPayload};
