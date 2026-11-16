//! Helper untuk membuat, membaca, dan menghapus session login.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/auth/session.md)
use tower_sessions::Session;

/// Nama key yang menyimpan username di dalam session server-side.
pub const SESSION_USERNAME_KEY: &str = "username";

#[derive(Clone, Debug)]
pub struct SessionIdentity {
    pub username: String,
}

/// Ganti ID session setelah login lalu simpan identitas pengguna.
pub async fn mulai_sesi(
    session: &Session,
    username: &str,
) -> Result<(), tower_sessions::session::Error> {
    // Mengganti ID mencegah session pra-login dipakai ulang setelah autentikasi.
    session.cycle_id().await?;
    session.insert(SESSION_USERNAME_KEY, username).await
}

/// Membaca username yang disimpan dalam session.
pub async fn identitas_sesi(
    session: &Session,
) -> Result<Option<SessionIdentity>, tower_sessions::session::Error> {
    let Some(username) = session.get(SESSION_USERNAME_KEY).await? else {
        return Ok(None);
    };
    Ok(Some(SessionIdentity { username }))
}

/// Menghapus session di store dan membersihkan data session lokal.
pub async fn hapus_sesi(session: &Session) -> Result<(), tower_sessions::session::Error> {
    session.flush().await
}
