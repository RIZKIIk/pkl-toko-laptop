//! Aturan input yang dipakai bersama oleh endpoint akun.

/// Membersihkan username dan memastikan sesuai batas kolom database.
pub(super) fn validasi_nama_pengguna(username: &str) -> Result<&str, &'static str> {
    let username = username.trim();
    if username.is_empty() {
        return Err("Username wajib diisi");
    }
    if username.chars().count() > 100 {
        return Err("Username maksimal 100 karakter");
    }
    Ok(username)
}
