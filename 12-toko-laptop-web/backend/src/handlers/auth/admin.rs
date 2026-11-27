//! Handler administrasi akun dan role pengguna.
use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use sqlx::PgPool;

use crate::auth::{self, middleware::AuthIdentity, response, role::UserRole};

use super::{
    payload::{CreateUserPayload, UpdateUserRolePayload},
    shared::validasi_nama_pengguna,
};

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

/// Memberi UI admin daftar role yang dapat dipilih saat membuat akun.
pub async fn daftar_peran_admin() -> Response {
    let roles = UserRole::dapat_dibuat_admin()
        .iter()
        .map(|role| {
            json!({
                "value": role.sebagai_teks(),
                "label": role.nama_tampilan(),
                "description": role.deskripsi()
            })
        })
        .collect::<Vec<_>>();

    response::respons_berhasil(json!({
        "status": "success",
        "data": { "roles": roles }
    }))
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

    match auth::ubah_peran_pengguna(&pool, username, payload.role).await {
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
