//! Bentuk data laptop untuk hasil query dan body request API.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/models/laptop/mod.md)
mod create_payload;
mod entity;
mod update_payload;

pub use create_payload::CreateLaptopPayload;
pub use entity::Laptop;
pub use update_payload::UpdateLaptopPayload;
