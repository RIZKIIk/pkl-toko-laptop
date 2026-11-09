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
pub async fn start_session(
    session: &Session,
    username: &str,
) -> Result<(), tower_sessions::session::Error> {
    // Mengganti ID mencegah session pra-login dipakai ulang setelah autentikasi.
    session.cycle_id().await?;
    session.insert(SESSION_USERNAME_KEY, username).await
}

/// Membaca username yang disimpan dalam session.
pub async fn session_identity(
    session: &Session,
) -> Result<Option<SessionIdentity>, tower_sessions::session::Error> {
    let Some(username) = session.get(SESSION_USERNAME_KEY).await? else {
        return Ok(None);
    };
    Ok(Some(SessionIdentity { username }))
}

/// Menghapus session di store dan membersihkan data session lokal.
pub async fn clear_session(session: &Session) -> Result<(), tower_sessions::session::Error> {
    session.flush().await
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tower_sessions::{MemoryStore, Session};

    use super::{clear_session, session_identity, start_session};

    #[tokio::test]
    async fn session_helpers_start_read_and_clear_login() {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(None, store, None);

        start_session(&session, "admin").await.unwrap();
        session.save().await.unwrap();
        assert_eq!(
            session_identity(&session)
                .await
                .unwrap()
                .map(|identity| identity.username),
            Some("admin".to_owned())
        );

        clear_session(&session).await.unwrap();
        assert!(session_identity(&session).await.unwrap().is_none());
    }
}
