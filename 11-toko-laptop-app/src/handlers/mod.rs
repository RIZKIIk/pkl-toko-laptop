//! Handler HTTP dikelompokkan menurut resource dan operasi request.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/handlers/mod.md)
pub mod auth;
pub mod laptops;
pub mod system;

pub use auth::{create_user, current_user, login, logout, register, update_user_role};
pub use laptops::{create_laptop, delete_laptop, get_laptop, list_laptops, update_laptop};
pub use system::health;
