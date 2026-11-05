//! Handler untuk login, pemeriksaan identitas, dan logout session/JWT admin.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/handlers/auth.md)
use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use tower_sessions::Session;

use crate::auth::{self, http, jwt::JwtService, role::UserRole, session};

/// Membersihkan username dan memastikan sesuai batas kolom database.
fn validate_username(username: &str) -> Result<&str, &'static str> {
    let username = username.trim();
    if username.is_empty() {
        return Err("Username wajib diisi");
    }
    if username.chars().count() > 100 {
        return Err("Username maksimal 100 karakter");
    }
    Ok(username)
}

/// Body JSON yang wajib dikirim ke `POST /api/auth/login`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoginPayload {
    /// Nama akun yang akan dicocokkan dengan tabel `users`.
    username: String,
    /// Password latihan yang akan diverifikasi; tidak dikirim kembali ke client.
    password: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateUserPayload {
    username: String,
    password: String,
    role: UserRole,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateUserRolePayload {
    role: UserRole,
}

/// Memeriksa kredensial lalu membuat session baru dan access token JWT.
pub async fn login(
    State(pool): State<PgPool>,
    Extension(jwt): Extension<JwtService>,
    session: Session,
    Json(payload): Json<LoginPayload>,
) -> Response {
    let username = match validate_username(&payload.username) {
        Ok(username) => username,
        Err(message) => return http::error_response(StatusCode::BAD_REQUEST, message),
    };
    if payload.password.is_empty() {
        return http::error_response(StatusCode::BAD_REQUEST, "Password wajib diisi");
    }

    let credentials_match = match auth::verify_credentials(&pool, username, &payload.password).await
    {
        Ok(matches) => matches,
        Err(error) => {
            eprintln!("Gagal memverifikasi kredensial user: {error}");
            return http::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat login",
            );
        }
    };

    if !credentials_match {
        return http::error_response(StatusCode::UNAUTHORIZED, "Username atau password salah");
    }

    // Satu login menghasilkan JWT dan session cookie untuk dua cara autentikasi.
    let (user_id, role) =
        match crate::repositories::user_repository::find_login_identity(&pool, username).await {
            Ok(Some(login_identity)) => login_identity,
            Ok(None) => {
                return http::error_response(
                    StatusCode::UNAUTHORIZED,
                    "Username atau password salah",
                );
            }
            Err(error) => {
                eprintln!("Gagal mengambil identitas user: {error}");
                return http::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Terjadi kesalahan saat login",
                );
            }
        };

    let token = match jwt.issue_token(user_id, role) {
        Ok(token) => token,
        Err(error) => {
            eprintln!("Gagal membuat JWT saat login: {error}");
            return http::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat membuat token login",
            );
        }
    };

    if let Err(error) = session::start_session(&session, username).await {
        eprintln!("Gagal membuat session login: {error}");
        return http::error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Terjadi kesalahan pada session",
        );
    }

    http::success_response(json!({
        "status": "success",
        "message": "Login berhasil",
        "data": {
            "username": username,
            "role": role,
            "access_token": token,
            "token_type": "Bearer",
            "expires_in": auth::jwt::TOKEN_EXPIRED_SECONDS
        }
    }))
}

/// Membuat akun baru dari username dan password yang dikirim sebagai JSON.
pub async fn register(State(pool): State<PgPool>, Json(payload): Json<LoginPayload>) -> Response {
    let username = match validate_username(&payload.username) {
        Ok(username) => username,
        Err(message) => return http::error_response(StatusCode::BAD_REQUEST, message),
    };
    if payload.password.is_empty() {
        return http::error_response(StatusCode::BAD_REQUEST, "Password wajib diisi");
    }

    match auth::register_user(&pool, username, &payload.password).await {
        Ok(true) => (
            StatusCode::CREATED,
            axum::Json(json!({
                "status": "success",
                "message": "Akun berhasil dibuat",
                "data": { "username": username, "role": UserRole::Buyer }
            })),
        )
            .into_response(),
        Ok(false) => http::error_response(StatusCode::CONFLICT, "Username sudah digunakan"),
        Err(error) => {
            eprintln!("Gagal membuat akun baru: {error}");
            http::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat membuat akun",
            )
        }
    }
}

/// Membuat akun buyer/restocker melalui endpoint khusus admin.
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserPayload>,
) -> Response {
    let username = match validate_username(&payload.username) {
        Ok(username) => username,
        Err(message) => return http::error_response(StatusCode::BAD_REQUEST, message),
    };
    if payload.password.is_empty() {
        return http::error_response(StatusCode::BAD_REQUEST, "Password wajib diisi");
    }
    if payload.role == UserRole::Admin {
        return http::error_response(
            StatusCode::BAD_REQUEST,
            "Role admin hanya dapat ditetapkan melalui konfigurasi server",
        );
    }

    match auth::register_user_with_role(&pool, username, &payload.password, payload.role).await {
        Ok(true) => (
            StatusCode::CREATED,
            Json(json!({
                "status": "success",
                "message": "Akun berhasil dibuat",
                "data": { "username": username, "role": payload.role }
            })),
        )
            .into_response(),
        Ok(false) => http::error_response(StatusCode::CONFLICT, "Username sudah digunakan"),
        Err(error) => {
            eprintln!("Gagal membuat akun dengan role: {error}");
            http::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat membuat akun",
            )
        }
    }
}

/// Mengubah role pengguna menjadi buyer/restocker dan mencabut kredensial lamanya.
pub async fn update_user_role(
    State(pool): State<PgPool>,
    Extension(identity): Extension<session::AuthIdentity>,
    Path(username): Path<String>,
    Json(payload): Json<UpdateUserRolePayload>,
) -> Response {
    let username = match validate_username(&username) {
        Ok(username) => username,
        Err(message) => return http::error_response(StatusCode::BAD_REQUEST, message),
    };
    if username == identity.username {
        return http::error_response(
            StatusCode::BAD_REQUEST,
            "Role akun yang sedang digunakan tidak dapat diubah",
        );
    }
    if payload.role == UserRole::Admin {
        return http::error_response(
            StatusCode::BAD_REQUEST,
            "Role admin hanya dapat ditetapkan melalui konfigurasi server",
        );
    }

    match crate::repositories::user_repository::set_user_role(&pool, username, payload.role).await {
        Ok(true) => http::success_response(json!({
            "status": "success",
            "message": "Role pengguna berhasil diperbarui",
            "data": { "username": username, "role": payload.role }
        })),
        Ok(false) => http::error_response(StatusCode::NOT_FOUND, "Pengguna tidak ditemukan"),
        Err(error) => {
            eprintln!("Gagal mengubah role pengguna: {error}");
            http::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat mengubah role pengguna",
            )
        }
    }
}

/// Mengembalikan identitas dari JWT bearer atau session aktif.
pub async fn current_user(Extension(identity): Extension<session::AuthIdentity>) -> Response {
    http::success_response(json!({
        "status": "success",
        "data": { "username": identity.username, "role": identity.role }
    }))
}

/// Mencabut JWT yang dipakai request ini dan menghapus session aktif.
pub async fn logout(
    State(pool): State<PgPool>,
    Extension(identity): Extension<session::AuthIdentity>,
    session: Session,
) -> Response {
    if let Some((jti, expires_at)) = &identity.token_to_revoke {
        if let Err(error) =
            crate::repositories::jwt_repository::revoke(&pool, jti, *expires_at).await
        {
            eprintln!("Gagal mencabut JWT saat logout: {error}");
            return http::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat logout",
            );
        }
    }

    if let Err(error) = auth::session::clear_session(&session).await {
        eprintln!("Gagal menghapus session saat logout: {error}");
        return http::error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Terjadi kesalahan pada session",
        );
    }

    http::success_response(json!({
        "status": "success",
        "message": "Logout berhasil"
    }))
}

#[cfg(test)]
mod tests {
    use super::validate_username;

    #[test]
    fn username_is_trimmed_before_use() {
        assert_eq!(validate_username("  rizki  "), Ok("rizki"));
    }

    #[test]
    fn username_must_not_be_empty() {
        assert_eq!(validate_username("   "), Err("Username wajib diisi"));
    }

    #[test]
    fn username_cannot_exceed_database_limit() {
        assert!(validate_username(&"a".repeat(100)).is_ok());
        assert_eq!(
            validate_username(&"a".repeat(101)),
            Err("Username maksimal 100 karakter")
        );
    }
}
