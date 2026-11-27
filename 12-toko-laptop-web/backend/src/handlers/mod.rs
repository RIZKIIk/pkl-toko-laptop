//! Handler HTTP dikelompokkan menurut resource dan operasi request.
//! Catatan: [Buka penjelasan](../../catatan/rust/handlers/mod.md)
pub mod auth;
pub mod laptops;
pub mod system;

// Jalur lama dipertahankan agar pemakai crate tidak perlu bermigrasi sekaligus.
pub use auth::{
    buat_pengguna, daftar, daftar_peran_admin, keluar, masuk, pengguna_saat_ini,
    ubah_peran_pengguna,
};
pub use laptops::{
    ambil_laptop, beli_laptop, daftar_laptop, hapus_laptop, tambah_laptop, ubah_laptop,
};
pub use system::cek_kesehatan;
