//! Library inti aplikasi toko laptop yang dipakai oleh binary server dan test.
//! [Buka catatan `lib.rs`](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/lib.md)

pub mod config;
pub mod error;
// Autentikasi: sinkronisasi akun latihan, verifikasi kredensial, dan session.
pub mod auth;
// Handler: fungsi yang menerima request HTTP dan menyusun response.
pub mod handlers;
// Model: bentuk data database serta payload JSON request.
pub mod models;
// Presenter: bentuk data response API, termasuk format harga Rupiah.
pub mod presenters;
// Repository: query untuk membaca dan mengubah data PostgreSQL.
pub mod repositories;
// Route: pendaftaran pasangan URL/method HTTP ke handler.
pub mod routes;
// Validation: pemeriksaan input sebelum query database dijalankan.
pub mod validation;
