//! Helper untuk membuat, membaca, dan menghapus session login.
//! Catatan: [Buka penjelasan](../../catatan/rust/auth/session.md)
use tower_sessions::Session;

/// Nama key yang menyimpan username di dalam session server-side.
pub const SESSION_USERNAME_KEY: &str = "username";
/// Nama key ID JWT yang harus dicabut saat session ini logout.
pub const SESSION_JWT_ID_KEY: &str = "jwt_id";

#[derive(Clone, Debug)]
pub struct SessionIdentity {
    pub username: String,
    pub jwt_id: Option<String>,
}

/// Ganti ID session setelah login lalu simpan identitas pengguna.
pub async fn mulai_sesi(
    session: &Session,
    username: &str,
    jwt_id: &str,
) -> Result<(), tower_sessions::session::Error> {
    // Mengganti ID mencegah session pra-login dipakai ulang setelah autentikasi.
    session.cycle_id().await?;
    session.insert(SESSION_USERNAME_KEY, username).await?;
    session.insert(SESSION_JWT_ID_KEY, jwt_id).await
}

/// Membaca username yang disimpan dalam session.
pub async fn identitas_sesi(
    session: &Session,
) -> Result<Option<SessionIdentity>, tower_sessions::session::Error> {
    let Some(username) = session.get(SESSION_USERNAME_KEY).await? else {
        return Ok(None);
    };
    let jwt_id = session.get(SESSION_JWT_ID_KEY).await?;
    Ok(Some(SessionIdentity { username, jwt_id }))
}

/// Menghapus session di store dan membersihkan data session lokal.
pub async fn hapus_sesi(session: &Session) -> Result<(), tower_sessions::session::Error> {
    session.flush().await
}
