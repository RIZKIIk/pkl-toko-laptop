# `src/auth/session.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

![Ilustrasi: Dua jalur autentikasi request](../../ilustrasi/halaman/10-session.svg "Dua jalur autentikasi request")


[Buka source `session.rs`](../../../src/auth/session.rs)

## Tujuan

File ini mengelola identitas session dan middleware autentikasi untuk endpoint yang memerlukan login. Middleware menerima cookie session atau JWT bearer yang belum kedaluwarsa maupun dicabut.

## Di halaman ini

[Alur login](#alur-autentikasi) · [Helper session](#helper-session) · [Middleware](#middleware-require_auth) · [Batasan store](#penyimpanan-dan-batasannya) · [Contoh curl](#contoh-memakai-cookie) · [File terkait](#file-terkait)

| Bagian | Tugas |
| --- | --- |
| `SESSION_USERNAME_KEY` | Menyamakan nama key username di session. |
| `start_session` | Mengganti ID session setelah login dan menyimpan username. |
| `session_identity` | Membaca username dari session request. |
| `clear_session` | Menghapus isi session dan record pada store. |
| `require_auth` | Meloloskan request dengan JWT atau session yang cocok dengan pengguna di database. |

## Alur autentikasi

```text
Login berhasil
  ├─ start_session: ganti ID session dan simpan username
  └─ handler menerbitkan JWT
       ↓
Request ke endpoint protected
  ├─ JWT valid + ID dan role cocok → teruskan request
  └─ JWT tidak ada/tidak valid → periksa username pada session
       ├─ username masih ada → teruskan request dengan role terbaru
       └─ session kosong/kedaluwarsa/dicabut → 401 Unauthorized
```

JWT diperiksa lebih dulu. Jika signature dan expiry valid, middleware memastikan `jti` belum dicabut, mencari user berdasarkan ID token, lalu mencocokkan role dengan database. Jika token tidak ada atau tidak valid, middleware mencoba session cookie dan mencari user berdasarkan username. Kegagalan teknis saat membaca session atau database menghasilkan `500`; kredensial yang kosong atau dicabut menghasilkan `401`.

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

pub const SESSION_USERNAME_KEY: &str = "username";
```

- `Extension<JwtService>` menerima verifier JWT yang dipasang router.
- `State<PgPool>` menyediakan database untuk memeriksa pengguna dan daftar JWT yang dicabut.
- `Request` dan `Next` mewakili request saat ini dan langkah handler berikutnya.
- `Session` memberi akses ke session server-side untuk request ini.
- `error_response` membentuk body JSON error yang seragam.
- `SESSION_USERNAME_KEY` menjaga agar semua fungsi memakai ejaan key yang sama.

## Helper session

### `start_session(session, username)`

```rust
session.cycle_id().await?;
session.insert(SESSION_USERNAME_KEY, username).await
```

Fungsi ini dipanggil setelah kredensial cocok. `cycle_id()` mengganti ID session untuk mengurangi risiko session fixation. Sesudah itu, `insert` menyimpan username. `?` meneruskan error jika operasi session gagal.

### `session_identity(session)`

Mengambil username dari session. Nilai `Some` berarti session memiliki identitas.

### `clear_session(session)`

Memanggil `session.flush()` untuk menghapus data session aktif dari store dan membersihkan session request. Handler logout juga mencatat `jti` JWT yang dipakai ke tabel denylist agar token tersebut ditolak.

## Middleware `require_auth`

Signature fungsi menerima extension JWT, session, request, dan `Next`, lalu selalu menghasilkan Axum `Response`.

1. Middleware memverifikasi signature/expiry JWT, memeriksa denylist `jti`, mencari pengguna dari ID `sub`, lalu mencocokkan role token dengan role terbaru di database.
2. Jika JWT tidak valid atau tidak ada, `session_identity(&session).await` membaca username cookie dan mencari pengguna beserta role terbaru di database.
3. Jika salah satu kredensial cocok, middleware menyimpan identitas terverifikasi pada request lalu meneruskannya ke handler.
4. Jika tidak ada kredensial yang cocok, JWT dicabut, atau role JWT sudah berubah, middleware mengembalikan `401 Unauthorized`.
5. Error database atau session dicatat memakai `eprintln!`, lalu client menerima `500` dengan pesan umum. Rincian internal tidak dikirim ke client.

Middleware ini dipasang hanya pada grup route protected di [routes.rs](../routes.md). `/health`, registrasi, dan login tetap publik.

## Penyimpanan dan batasannya

Router memakai `MemoryStore`. Session akan hilang ketika server dimulai ulang dan tidak dibagikan ke instance aplikasi lain. Saat logout dengan JWT, aplikasi menyimpan `jti` token itu ke PostgreSQL sehingga token langsung ditolak sampai kedaluwarsa. Token lain milik pengguna dan session pada perangkat lain tidak ikut dicabut.

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
