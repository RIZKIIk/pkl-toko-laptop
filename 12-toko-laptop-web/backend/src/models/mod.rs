//! Model dan payload request yang digunakan bersama oleh lapisan backend.
//! Catatan: [Buka penjelasan](../../catatan/rust/models/mod.md)
pub mod laptop;
pub mod purchase;

pub use laptop::{CreateLaptopPayload, Laptop, UpdateLaptopPayload};
pub use purchase::PurchaseReceipt;
