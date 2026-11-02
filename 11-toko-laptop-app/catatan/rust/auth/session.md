# `src/auth/session.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

![Ilustrasi: Dua jalur autentikasi request](../../ilustrasi/halaman/10-session.svg "Dua jalur autentikasi request")


[Buka source `session.rs`](../../../src/auth/session.rs)

## Tujuan

File ini mengelola identitas session dan middleware autentikasi untuk endpoint yang memerlukan login. Middleware menerima salah satu dari dua bukti login: cookie session yang berisi username atau JWT bearer yang valid.

## Di halaman ini

[Alur login](#alur-autentikasi) · [Helper session](#helper-session) · [Middleware](#middleware-require_auth) · [Batasan store](#penyimpanan-dan-batasannya) · [Contoh curl](#contoh-memakai-cookie) · [File terkait](#file-terkait)

| Bagian | Tugas |
| --- | --- |
| `SESSION_USERNAME_KEY` | Menyamakan nama key username yang disimpan dan dibaca dari session. |
| `start_session` | Mengganti ID session setelah login dan menyimpan username. |
| `session_username` | Membaca username dari session request. |
| `clear_session` | Menghapus isi session dan record pada store. |
| `require_auth` | Meloloskan request dengan JWT atau session yang memiliki username. |

## Alur autentikasi

```text
Login berhasil
  ├─ start_session: ganti ID session dan simpan username
  └─ handler menerbitkan JWT
       ↓
Request ke endpoint protected
  ├─ JWT bearer valid → teruskan request
  └─ JWT tidak ada/tidak valid → periksa username pada session
       ├─ username ada → teruskan request
       └─ session kosong → 401 Unauthorized
```

JWT diperiksa lebih dulu. Jika tidak ada atau tidak valid, request masih dapat diterima lewat session cookie. Kegagalan teknis saat membaca session menghasilkan `500`; itu berbeda dari session yang terbaca tetapi belum login (`401`).

## Import dan konstanta

```rust
use axum::{
    extract::{Extension, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tower_sessions::Session;

use super::{http::error_response, jwt::JwtService};

pub(crate) const SESSION_USERNAME_KEY: &str = "username";
```

- `Extension<JwtService>` menerima verifier JWT yang dipasang router.
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

Fungsi ini dipanggil setelah kredensial cocok. `cycle_id()` mengganti ID session untuk mengurangi risiko session fixation. Sesudah itu, `insert` menyimpan username. Hasil `Result` terakhir dikembalikan langsung; `?` meneruskan error jika pergantian ID gagal.

### `session_username(session)`

Mengambil nilai bertipe `Option<String>` dari key username. `Ok(Some(username))` berarti sudah login, `Ok(None)` berarti session ada tetapi belum berisi identitas, dan `Err(error)` berarti store gagal dibaca.

### `clear_session(session)`

Memanggil `session.flush()` untuk menghapus data session dari store dan membersihkan session request. Handler logout menangani error dan mengembalikan response yang sesuai. JWT tidak dihapus oleh operasi ini.

## Middleware `require_auth`

Signature fungsi menerima extension JWT, session, request, dan `Next`, lalu selalu menghasilkan Axum `Response`.

1. `username_from_headers(request.headers(), &jwt)` mencari dan memverifikasi bearer token.
2. Jika token valid, `next.run(request).await` meneruskan request tanpa membaca session.
3. Jika tidak, `session_username(&session).await` memeriksa identitas di cookie session.
4. `Ok(Some(_))` meneruskan request. Username tidak perlu dibaca lagi oleh middleware karena handler berikutnya yang membutuhkan identitas mengambilnya sendiri.
5. `Ok(None)` mengembalikan `401 Unauthorized` dengan pesan `Login diperlukan`.
6. `Err(error)` dicatat memakai `eprintln!`, lalu client menerima `500` dengan pesan umum. Rincian internal tidak dikirim ke client.

Middleware ini dipasang hanya pada grup route protected di [routes.rs](../routes.md). `/health`, registrasi, dan login tetap publik.

## Penyimpanan dan batasannya

Router memakai `MemoryStore`. Session akan hilang ketika server dimulai ulang dan tidak dibagikan ke instance aplikasi lain. Session dan JWT juga memiliki siklus logout berbeda: logout menghapus session, sedangkan JWT masih berlaku sampai expiry satu jam. Client harus menghapus token setelah logout.

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
