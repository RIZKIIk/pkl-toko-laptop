//! Handler autentikasi dan pengelolaan akun, dipisah menurut alur request.
//! Catatan: [Buka penjelasan](../../catatan/rust/handlers/auth.md)
mod admin;
mod login;
mod payload;
mod register;
mod session;
mod shared;

pub use admin::{buat_pengguna, daftar_peran_admin, ubah_peran_pengguna};
pub use login::masuk;
pub use payload::{CreateUserPayload, LoginPayload, RegisterPayload, UpdateUserRolePayload};
pub use register::daftar;
pub use session::{keluar, pengguna_saat_ini};
