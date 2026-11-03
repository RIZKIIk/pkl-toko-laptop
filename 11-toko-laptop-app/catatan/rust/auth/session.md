# `src/auth/session.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

![Ilustrasi: Dua jalur autentikasi request](../../ilustrasi/halaman/10-session.svg "Dua jalur autentikasi request")


[Buka source `session.rs`](../../../src/auth/session.rs)

## Tujuan

File ini mengelola identitas session dan middleware autentikasi untuk endpoint yang memerlukan login. Middleware menerima salah satu dari dua bukti login: cookie session atau JWT bearer yang versinya masih aktif.

## Di halaman ini

[Alur login](#alur-autentikasi) · [Helper session](#helper-session) · [Middleware](#middleware-require_auth) · [Batasan store](#penyimpanan-dan-batasannya) · [Contoh curl](#contoh-memakai-cookie) · [File terkait](#file-terkait)

| Bagian | Tugas |
| --- | --- |
| `SESSION_USERNAME_KEY` / `SESSION_AUTH_VERSION_KEY` | Menyamakan nama key identitas dan versi autentikasi di session. |
| `start_session` | Mengganti ID session setelah login dan menyimpan username serta versi autentikasi. |
| `session_identity` | Membaca username dan versi autentikasi dari session request. |
| `clear_session` | Menghapus isi session dan record pada store. |
| `require_auth` | Meloloskan request dengan JWT atau session yang versinya cocok dengan database. |

## Alur autentikasi

```text
Login berhasil
  ├─ start_session: ganti ID session dan simpan username + auth_version
  └─ handler menerbitkan JWT
       ↓
Request ke endpoint protected
  ├─ JWT valid + auth_version cocok → teruskan request
  └─ JWT tidak ada/tidak valid → periksa session dan auth_version
       ├─ auth_version cocok dengan database → teruskan request
       └─ session kosong/kedaluwarsa/dicabut → 401 Unauthorized
```

JWT diperiksa lebih dulu. Jika signature dan expiry valid, middleware mencocokkan versi autentikasi token dengan database. Jika token tidak ada, tidak valid, atau sudah dicabut, middleware mencoba session cookie dan melakukan pemeriksaan versi yang sama. Kegagalan teknis saat membaca session atau database menghasilkan `500`; kredensial yang kosong atau dicabut menghasilkan `401`.

## Import dan konstanta

```rust
use axum::{
    extract::{Extension, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tower_sessions::Session;
use sqlx::PgPool;

use super::{http::error_response, jwt::JwtService};

pub(crate) const SESSION_USERNAME_KEY: &str = "username";
pub(crate) const SESSION_AUTH_VERSION_KEY: &str = "auth_version";
```

- `Extension<JwtService>` menerima verifier JWT yang dipasang router.
- `State<PgPool>` menyediakan database untuk memeriksa versi autentikasi aktif.
- `Request` dan `Next` mewakili request saat ini dan langkah handler berikutnya.
- `Session` memberi akses ke session server-side untuk request ini.
- `error_response` membentuk body JSON error yang seragam.
- `SESSION_USERNAME_KEY` menjaga agar semua fungsi memakai ejaan key yang sama.
- `SESSION_AUTH_VERSION_KEY` menyimpan versi autentikasi saat session dibuat.

## Helper session

### `start_session(session, username, auth_version)`

```rust
session.cycle_id().await?;
session.insert(SESSION_USERNAME_KEY, username).await?;
session.insert(SESSION_AUTH_VERSION_KEY, auth_version).await
```

Fungsi ini dipanggil setelah kredensial cocok. `cycle_id()` mengganti ID session untuk mengurangi risiko session fixation. Sesudah itu, `insert` menyimpan username dan versi autentikasi saat login. `?` meneruskan error jika operasi session gagal.

### `session_identity(session)`

Mengambil username dan `auth_version` dari session. Nilai `Some` berarti session memiliki identitas; session lama tanpa versi autentikasi dianggap tidak terautentikasi.

### `clear_session(session)`

Memanggil `session.flush()` untuk menghapus data session aktif dari store dan membersihkan session request. Handler logout juga menaikkan `users.auth_version`, yang mencabut semua JWT dan session lama milik pengguna.

## Middleware `require_auth`

Signature fungsi menerima extension JWT, session, request, dan `Next`, lalu selalu menghasilkan Axum `Response`.

1. Middleware memverifikasi signature/expiry JWT dan mencocokkan `ver` dengan `auth_version` terbaru di database.
2. Jika JWT tidak valid atau tidak ada, `session_identity(&session).await` membaca identitas cookie lalu middleware memeriksa versinya di database.
3. Jika salah satu kredensial cocok, middleware menyimpan identitas terverifikasi pada request lalu meneruskannya ke handler.
4. Jika tidak ada kredensial yang cocok atau versinya sudah dicabut, middleware mengembalikan `401 Unauthorized`.
5. Error database atau session dicatat memakai `eprintln!`, lalu client menerima `500` dengan pesan umum. Rincian internal tidak dikirim ke client.

Middleware ini dipasang hanya pada grup route protected di [routes.rs](../routes.md). `/health`, registrasi, dan login tetap publik.

## Penyimpanan dan batasannya

Router memakai `MemoryStore`. Session akan hilang ketika server dimulai ulang dan tidak dibagikan ke instance aplikasi lain. Saat logout, aplikasi menaikkan `auth_version` di database; semua cookie session dan JWT yang menyimpan versi sebelumnya langsung ditolak, termasuk pada request berikutnya. Client tetap perlu menghapus token setelah logout.

## Contoh memakai cookie

`curl` dapat menyimpan cookie login ke file lalu mengirimkannya pada request berikutnya:

```bash
curl -i -c cookies.txt \
  -H 'Content-Type: application/json' \
  -d '{"username":"admin","password":"PASSWORD_LOKAL"}' \
  http://127.0.0.1:3001/api/auth/login

curl -i -b cookies.txt http://127.0.0.1:3001/api/laptops
```

File cookie berisi kredensial aktif. Jangan dibagikan atau dimasukkan ke version control. Contoh seluruh request tersedia di [panduan curl](../../curl.md).

## File terkait

- [Handler auth](../handlers/auth.md) memulai dan menghapus session.
- [JWT](jwt.md) menjelaskan pembuatan dan pemeriksaan bearer token.
- [Routes](../routes.md) memasang `require_auth` dan `SessionManagerLayer`.
- Source berisi unit test helper untuk menyimpan, membaca, dan membersihkan session.
