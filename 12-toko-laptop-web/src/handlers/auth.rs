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

use crate::auth::{
    self, jwt::JwtService, middleware::AuthIdentity, response, role::UserRole, session,
};

/// Membersihkan username dan memastikan sesuai batas kolom database.
fn validasi_nama_pengguna(username: &str) -> Result<&str, &'static str> {
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
pub async fn masuk(
    State(pool): State<PgPool>,
    Extension(jwt): Extension<JwtService>,
    session: Session,
    Json(payload): Json<LoginPayload>,
) -> Response {
    let username = match validasi_nama_pengguna(&payload.username) {
        Ok(username) => username,
        Err(message) => return response::respons_gagal(StatusCode::BAD_REQUEST, message),
    };
    if payload.password.is_empty() {
        return response::respons_gagal(StatusCode::BAD_REQUEST, "Password wajib diisi");
    }

    let credentials_match =
        match auth::verifikasi_kredensial(&pool, username, &payload.password).await {
            Ok(matches) => matches,
            Err(error) => {
                eprintln!("Gagal memverifikasi kredensial user: {error}");
                return response::respons_gagal(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Terjadi kesalahan saat login",
                );
            }
        };

    if !credentials_match {
        return response::respons_gagal(StatusCode::UNAUTHORIZED, "Username atau password salah");
    }

    // Satu login menghasilkan JWT dan session cookie untuk dua cara autentikasi.
    let (user_id, role) =
        match crate::repositories::user_repository::cari_identitas_pengguna_untuk_login(
            &pool, username,
        )
        .await
        {
            Ok(Some(login_identity)) => login_identity,
            Ok(None) => {
                return response::respons_gagal(
                    StatusCode::UNAUTHORIZED,
                    "Username atau password salah",
                );
            }
            Err(error) => {
                eprintln!("Gagal mengambil identitas user: {error}");
                return response::respons_gagal(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Terjadi kesalahan saat login",
                );
            }
        };

    let token = match jwt.terbitkan_token(user_id) {
        Ok(token) => token,
        Err(error) => {
            eprintln!("Gagal membuat JWT saat login: {error}");
            return response::respons_gagal(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat membuat token login",
            );
        }
    };

    if let Err(error) = session::mulai_sesi(&session, username).await {
        eprintln!("Gagal membuat session login: {error}");
        return response::respons_gagal(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Terjadi kesalahan pada session",
        );
    }

    response::respons_berhasil(json!({
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
pub async fn daftar(State(pool): State<PgPool>, Json(payload): Json<LoginPayload>) -> Response {
    let username = match validasi_nama_pengguna(&payload.username) {
        Ok(username) => username,
        Err(message) => return response::respons_gagal(StatusCode::BAD_REQUEST, message),
    };
    if payload.password.is_empty() {
        return response::respons_gagal(StatusCode::BAD_REQUEST, "Password wajib diisi");
    }

    match auth::daftarkan_pengguna(&pool, username, &payload.password).await {
        Ok(true) => (
            StatusCode::CREATED,
            axum::Json(json!({
                "status": "success",
                "message": "Akun berhasil dibuat",
                "data": { "username": username, "role": UserRole::Buyer }
            })),
        )
            .into_response(),
        Ok(false) => response::respons_gagal(StatusCode::CONFLICT, "Username sudah digunakan"),
        Err(error) => {
            eprintln!("Gagal membuat akun baru: {error}");
            response::respons_gagal(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat membuat akun",
            )
        }
    }
}

/// Membuat akun buyer/restocker melalui endpoint khusus admin.
pub async fn buat_pengguna(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserPayload>,
) -> Response {
    let username = match validasi_nama_pengguna(&payload.username) {
        Ok(username) => username,
        Err(message) => return response::respons_gagal(StatusCode::BAD_REQUEST, message),
    };
    if payload.password.is_empty() {
        return response::respons_gagal(StatusCode::BAD_REQUEST, "Password wajib diisi");
    }
    if payload.role == UserRole::Admin {
        return response::respons_gagal(
            StatusCode::BAD_REQUEST,
            "Role admin hanya dapat ditetapkan melalui konfigurasi server",
        );
    }

    match auth::daftarkan_pengguna_dengan_peran(&pool, username, &payload.password, payload.role)
        .await
    {
        Ok(true) => (
            StatusCode::CREATED,
            Json(json!({
                "status": "success",
                "message": "Akun berhasil dibuat",
                "data": { "username": username, "role": payload.role }
            })),
        )
            .into_response(),
        Ok(false) => response::respons_gagal(StatusCode::CONFLICT, "Username sudah digunakan"),
        Err(error) => {
            eprintln!("Gagal membuat akun dengan role: {error}");
            response::respons_gagal(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat membuat akun",
            )
        }
    }
}

/// Mengubah role pengguna menjadi buyer/restocker tanpa membatalkan JWT aktif.
pub async fn ubah_peran_pengguna(
    State(pool): State<PgPool>,
    Extension(identity): Extension<AuthIdentity>,
    Path(username): Path<String>,
    Json(payload): Json<UpdateUserRolePayload>,
) -> Response {
    let username = match validasi_nama_pengguna(&username) {
        Ok(username) => username,
        Err(message) => return response::respons_gagal(StatusCode::BAD_REQUEST, message),
    };
    if username == identity.username {
        return response::respons_gagal(
            StatusCode::BAD_REQUEST,
            "Role akun yang sedang digunakan tidak dapat diubah",
        );
    }
    if payload.role == UserRole::Admin {
        return response::respons_gagal(
            StatusCode::BAD_REQUEST,
            "Role admin hanya dapat ditetapkan melalui konfigurasi server",
        );
    }

    match crate::repositories::user_repository::ubah_peran_pengguna(&pool, username, payload.role)
        .await
    {
        Ok(true) => response::respons_berhasil(json!({
            "status": "success",
            "message": "Role pengguna berhasil diperbarui",
            "data": { "username": username, "role": payload.role }
        })),
        Ok(false) => response::respons_gagal(StatusCode::NOT_FOUND, "Pengguna tidak ditemukan"),
        Err(error) => {
            eprintln!("Gagal mengubah role pengguna: {error}");
            response::respons_gagal(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan saat mengubah role pengguna",
            )
        }
    }
}

/// Mengembalikan identitas dari JWT bearer atau session aktif.
pub async fn pengguna_saat_ini(Extension(identity): Extension<AuthIdentity>) -> Response {
    response::respons_berhasil(json!({
        "status": "success",
        "data": { "username": identity.username, "role": identity.role }
    }))
}

/// Menghapus session aktif; JWT tetap berlaku hingga mencapai waktu kedaluwarsanya.
pub async fn keluar(session: Session) -> Response {
    if let Err(error) = auth::session::hapus_sesi(&session).await {
        eprintln!("Gagal menghapus session saat logout: {error}");
        return response::respons_gagal(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Terjadi kesalahan pada session",
        );
    }

    response::respons_berhasil(json!({
        "status": "success",
        "message": "Logout berhasil"
    }))
}
