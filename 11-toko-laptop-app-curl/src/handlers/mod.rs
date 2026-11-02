//! Handler HTTP dikelompokkan menurut resource dan operasi request.
//! Dokumentasi: file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/handlers/mod.md
pub mod auth;
pub mod laptops;
pub mod system;

// Ekspor handler di tingkat modul agar route menggunakan satu namespace.
pub use auth::{current_user, login, logout, register};
pub use laptops::{create_laptop, delete_laptop, get_laptop, list_laptops, update_laptop};
pub use system::health;
