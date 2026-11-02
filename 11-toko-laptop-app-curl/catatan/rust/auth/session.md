# `src/auth/session.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

[Buka source `session.rs`](../../../src/auth/session.rs)

## Gambaran singkat

File ini menyimpan helper session dan middleware untuk route yang memerlukan login.

```text
login berhasil -> start_session menyimpan username
request protected -> require_auth membaca username
logout -> clear_session menghapus session
```

Session disimpan di server. Browser atau client membawa cookie session; username tidak dipercaya dari body request untuk setiap operasi. Route dan middleware dipasang di [routes](../routes.md), sedangkan handler login/logout ada di [handler auth](../handlers/auth.md).

## 1. Import

```rust
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use tower_sessions::Session;
use super::http::error_response;
```

- `Request` mewakili request HTTP yang diteruskan ke middleware.
- `StatusCode` menyediakan kode seperti `401 Unauthorized` dan `500 Internal Server Error`.
- `Next` mewakili langkah middleware/handler berikutnya dalam rantai request.
- `Response` adalah hasil HTTP yang dikembalikan middleware.
- `Session` adalah handle dari `tower-sessions` untuk membaca dan menulis data session.
- `super::http::error_response` memakai helper response dari modul `auth` induk.

`use` membawa nama ke cakupan file. `super::` berarti mulai dari modul induk saat ini, yaitu `auth`.

## 2. Key username session

```rust
pub(crate) const SESSION_USERNAME_KEY: &str = "username";
```

- `pub(crate)` membuka konstanta ini ke bagian lain dalam crate, tetapi bukan ke pemakai crate dari luar.
- `const` membuat nilai konstan yang tidak berubah saat program berjalan.
- `SESSION_USERNAME_KEY` adalah nama konstanta; huruf kapital dan underscore adalah konvensi Rust untuk konstanta.
- `: &str` menyatakan bahwa nilainya berupa string slice.

Handler memakai key yang sama saat menyimpan dan membaca username agar tidak terjadi perbedaan ejaan key.

## 3. `start_session`

```rust
pub async fn start_session(
    session: &Session,
    username: &str,
) -> Result<(), tower_sessions::session::Error> {
    session.cycle_id().await?;
    session.insert(SESSION_USERNAME_KEY, username).await
}
```

Fungsi asinkron ini memulai identitas login pada session.

- `&Session` dan `&str` adalah referensi: fungsi meminjam session dan teks username.
- `async` memungkinkan fungsi menunggu operasi session dengan `.await`.
- `Result<(), tower_sessions::session::Error>` berarti sukses tanpa nilai tambahan, atau error dari penyimpanan session.
- `session.cycle_id().await?` mengganti ID session setelah autentikasi. `?` menghentikan fungsi dan meneruskan error jika pergantian ID gagal.
- `session.insert(SESSION_USERNAME_KEY, username).await` menyimpan username dengan key tetap. Ini adalah ekspresi terakhir, jadi hasil `Result` langsung dikembalikan.

Penggantian ID membantu mengurangi risiko session fixation. Detail teknis session tetap dikelola oleh library.

## 4. `session_username`

```rust
pub async fn session_username(
    session: &Session,
) -> Result<Option<String>, tower_sessions::session::Error> {
    session.get(SESSION_USERNAME_KEY).await
}
```

Fungsi ini mengambil username dari session:

- `Option<String>` menunjukkan username bisa ditemukan (`Some(username)`) atau belum ada (`None`).
- `Result<..., Error>` membedakan session kosong dari kegagalan membaca store. `Ok(None)` adalah kondisi normal belum login; `Err(...)` menunjukkan masalah teknis.
- `session.get(...).await` meminta nilai dengan key username dan langsung mengembalikan hasilnya.

## 5. `clear_session`

```rust
pub async fn clear_session(session: &Session) -> Result<(), tower_sessions::session::Error> {
    session.flush().await
}
```

`session.flush().await` menghapus data session lokal dan record session di store. Hasil operasi berupa `Result`; handler dapat mencatat error jika penghapusan gagal.

## 6. Middleware `require_auth`

```rust
pub async fn require_auth(session: Session, request: Request, next: Next) -> Response {
    match session_username(&session).await {
        Ok(Some(_)) => next.run(request).await,
        Ok(None) => error_response(StatusCode::UNAUTHORIZED, "Login diperlukan"),
        Err(error) => {
            eprintln!("Gagal membaca session: {error}");
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan pada session",
            )
        }
    }
}
```

### Parameter dan hasil

- `session: Session` adalah session request saat ini.
- `request: Request` adalah request yang akan diteruskan jika pengguna sudah login.
- `next: Next` adalah jalur untuk melanjutkan ke handler berikutnya.
- `-> Response` berarti middleware selalu menghasilkan response HTTP, baik dengan meneruskan request maupun menghentikannya dengan error.

### Cara membaca `match`

`match` memilih tindakan berdasarkan hasil `session_username(...).await`:

| Pola | Arti | Tindakan |
| --- | --- | --- |
| `Ok(Some(_))` | Session berhasil dibaca dan ada username. `_` berarti nilainya ada tetapi tidak perlu dipakai di sini. | `next.run(request).await` meneruskan request ke handler berikutnya. |
| `Ok(None)` | Session terbaca, tetapi tidak menyimpan username. | Mengirim `401 Unauthorized` dengan pesan login diperlukan. |
| `Err(error)` | Pembacaan session gagal secara teknis. | Mencatat detail di log server lalu mengirim `500` dengan pesan umum. |

`eprintln!` mencetak error ke stderr server. Detail error tidak dikirim ke client; client hanya menerima pesan aman melalui [`error_response`](http.md).

## 7. Unit test

Test pada source membuat `MemoryStore` sementara, membuat `Session`, kemudian:

1. Memulai session untuk `"admin"`.
2. Menyimpan perubahan session dengan `session.save().await`.
3. Memastikan `session_username` membaca kembali `"admin"`.
4. Memanggil `clear_session` dan memastikan hasil bacaan menjadi `None`.

`#[cfg(test)]` menyertakan modul hanya saat test dikompilasi. `#[tokio::test]` menyediakan runtime asynchronous untuk fungsi test, `Arc` memungkinkan store dibagikan, dan `assert_eq!` membandingkan hasil aktual dengan nilai yang diharapkan.

## Catatan penyimpanan

Router saat ini menggunakan `MemoryStore`, sehingga session hilang ketika proses server dimulai ulang dan tidak dibagikan antar-instance server. Password yang diperiksa login juga disimpan sebagai teks biasa untuk latihan lokal; keduanya bukan rancangan produksi.
