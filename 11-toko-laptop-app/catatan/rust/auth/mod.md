# `src/auth/mod.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

[Buka source](../../../src/auth/mod.rs)

## Gambaran singkat

Modul `auth` mengatur layanan autentikasi: menyiapkan akun admin, mendaftarkan akun, memeriksa username dan password, serta menyediakan middleware untuk melindungi request.

Modul ini tidak menangani query SQL secara langsung. Operasi tabel `users` diteruskan ke [user repository](../repositories/user-repository.md). Pembuatan response HTTP auth berada di [`http.rs`](http.md), pengelolaan session berada di [`session.rs`](session.md), dan JWT HS256 berada di [`jwt.rs`](jwt.md).

```text
src/auth/
├── mod.rs      # Layanan akun dan pemeriksaan kredensial
├── http.rs     # Pembentuk response JSON auth
├── jwt.rs      # Penerbitan dan verifikasi JWT HS256
└── session.rs  # Helper session dan middleware autentikasi
```

## 1. Deklarasi modul

Di awal file:

```rust
pub mod http;
pub mod jwt;
pub mod session;
```

- `mod http;` memberi tahu Rust bahwa isi modul `http` ada di file `http.rs`.
- `mod jwt;` memberi tahu Rust bahwa penerbitan dan verifikasi token ada di `jwt.rs`.
- `mod session;` memberi tahu Rust bahwa isi modul `session` ada di file `session.rs`.
- `pub` membuat nama modul dapat diakses melalui jalur publik, misalnya `auth::http`.

Setiap modul memiliki tanggung jawab berbeda. Lihat [catatan `http.rs`](http.md) untuk response JSON, [catatan `jwt.rs`](jwt.md) untuk access token, dan [catatan `session.rs`](session.md) untuk session serta pemeriksaan login.

## 2. Import dan re-export

```rust
use std::error::Error;
use sqlx::PgPool;
use crate::repositories::user_repository;

pub use session::require_auth;
```

| Sintaks | Kegunaan |
| --- | --- |
| `use std::error::Error;` | Mengimpor trait `Error` dari pustaka standar Rust untuk menuliskan tipe error umum. |
| `use sqlx::PgPool;` | Mengimpor pool koneksi PostgreSQL yang dipakai untuk mengakses database. Pool mengelola koneksi agar dapat digunakan kembali. |
| `use crate::repositories::user_repository;` | Mengimpor modul repository user dari crate aplikasi ini. Modul tersebut menjalankan query ke tabel `users`. |
| `pub use session::require_auth;` | Mengimpor sekaligus mengekspor ulang `require_auth` dari modul `session`. Kode lain dapat mengaksesnya melalui `auth::require_auth`, tanpa menulis `auth::session::require_auth`. |

`crate::` berarti jalur dimulai dari akar crate aplikasi ini. `use` membuat nama yang sering dipakai lebih ringkas. `pub use` juga menjadikan nama yang diimpor tersedia bagi pemanggil dari luar modul, sesuai batas visibilitas modul.

## 3. `initialize_admin_user`

Kode:

```rust
pub async fn initialize_admin_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    user_repository::upsert_user(pool, username, password).await?;
    Ok(())
}
```

### Tujuan penyiapan akun admin

Fungsi ini menyiapkan akun admin dari konfigurasi aplikasi saat startup. Fungsi meneruskan data ke repository untuk menyimpan akun. Jika username sudah ada, operasi *upsert* memperbarui password akun tersebut.

### Cara membaca deklarasinya

- `pub` membuat fungsi bisa dipanggil dari bagian lain crate maupun pemakai crate melalui jalur publik.
- `async` menandai fungsi asinkron. Operasi database dapat menunggu I/O tanpa memblokir thread secara langsung.
- `fn initialize_admin_user` mendeklarasikan fungsi bernama `initialize_admin_user`.
- `pool: &PgPool` adalah referensi ke pool koneksi; tanda `&` berarti fungsi meminjamnya, bukan mengambil alih kepemilikannya.
- `username: &str` dan `password: &str` adalah referensi ke teks username dan password.
- `-> Result<(), Box<dyn Error + Send + Sync>>` berarti fungsi dapat berhasil tanpa mengembalikan data (`()`), atau gagal dengan error umum.
  - `Result<T, E>` mewakili hasil yang bisa sukses (`Ok(T)`) atau gagal (`Err(E)`).
  - `Box<dyn Error ...>` memungkinkan beragam jenis error disimpan melalui satu tipe error umum.
  - `Send + Sync` menyatakan syarat agar error aman dipindahkan dan dibagikan antar-thread, sesuai kebutuhan aplikasi asinkron.

### Cara membaca isi fungsi

1. `user_repository::upsert_user(...)` meminta repository menyimpan atau memperbarui akun. Pemisahan ini membuat fungsi auth mengatur alur, sementara repository mengatur query database.
2. `.await` menunggu operasi asinkron tersebut selesai.
3. `?` berarti: jika hasilnya `Err`, hentikan fungsi dan teruskan error ke pemanggil; jika `Ok`, lanjutkan.
4. `Ok(())` menyatakan operasi selesai sukses dan tidak ada nilai khusus yang perlu dikembalikan.

Fungsi ini dipanggil saat startup setelah database siap. Repository memakai `ON CONFLICT` sehingga username yang sudah ada tidak dibuat sebagai baris duplikat.

## 4. `register_user`

Kode:

```rust
pub async fn register_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<bool, sqlx::Error> {
    user_repository::insert_user(pool, username, password).await
}
```

Fungsi ini mencoba membuat akun baru tanpa menimpa akun dengan username yang sama.

- Parameter `pool`, `username`, dan `password` memiliki arti yang sama seperti di `initialize_admin_user`.
- `Result<bool, sqlx::Error>` menyatakan ada dua tingkat hasil:
  - `Ok(true)`: akun berhasil dibuat.
  - `Ok(false)`: username sudah digunakan, jadi tidak ada akun baru yang dibuat.
  - `Err(error)`: terjadi error database.
- `user_repository::insert_user(...)` menjalankan operasi insert.
- `.await` menunggu query selesai.

Baris terakhir tidak diakhiri titik koma, sehingga hasil repository langsung menjadi hasil fungsi. Tidak diperlukan `?` atau `Ok(...)` tambahan karena tipe hasil repository sama dengan tipe hasil fungsi ini.

## 5. `verify_credentials`

Kode:

```rust
pub async fn verify_credentials(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let Some(stored_password) = user_repository::find_password(pool, username).await? else {
        return Ok(false);
    };

    Ok(password_matches(&stored_password, password))
}
```

### Tujuannya

Fungsi ini mencari password yang tersimpan untuk username yang diberikan, lalu membandingkannya dengan password dari request.

### Alur baris demi baris

1. `user_repository::find_password(pool, username)` meminta repository mencari password berdasarkan username.
2. `.await` menunggu query database selesai.
3. `?` meneruskan error database ke pemanggil jika query gagal.
4. Hasil query berbentuk `Option<String>`:
   - `Some(password)` berarti username ditemukan.
   - `None` berarti username tidak ditemukan.
5. Sintaks `let Some(stored_password) = ... else { ... };` adalah *let-else*. Jika hasilnya `Some`, nilai password dimasukkan ke `stored_password` dan fungsi melanjutkan. Jika hasilnya `None`, blok `else` dijalankan.
6. `return Ok(false);` menghentikan fungsi dengan hasil “kredensial tidak cocok”. Username yang tidak ditemukan bukan error database.
7. Jika username ditemukan, `password_matches(...)` memeriksa password.
8. `Ok(...)` membungkus hasil pemeriksaan sebagai keberhasilan fungsi. Nilai `false` di dalam `Ok` artinya proses pemeriksaan berhasil dilakukan, tetapi kredensialnya tidak cocok.

Jadi `Ok(false)` berbeda dari `Err(error)`: yang pertama berarti login tidak cocok secara normal, sedangkan yang kedua berarti pemeriksaan gagal karena masalah seperti error database.

## 6. `password_matches`

```rust
fn password_matches(stored_password: &str, provided_password: &str) -> bool {
    !provided_password.is_empty() && stored_password == provided_password
}
```

Fungsi ini privat karena tidak memiliki `pub`; hanya kode dalam modul `auth` yang dapat memanggilnya.

- `stored_password` adalah password yang ditemukan di database.
- `provided_password` adalah password yang dikirim pengguna.
- `-> bool` berarti hasilnya hanya `true` atau `false`.
- `provided_password.is_empty()` memeriksa apakah input kosong.
- `!` berarti “bukan”, jadi `!provided_password.is_empty()` bernilai `true` jika password yang diberikan tidak kosong.
- `&&` berarti “dan”. Kedua kondisi di sebelah kiri dan kanan harus benar.
- `stored_password == provided_password` membandingkan kedua teks agar sama persis.

Artinya, password hanya dianggap cocok jika input tidak kosong dan teksnya sama persis dengan nilai yang tersimpan.

> **Catatan keamanan:** implementasi latihan ini menyimpan dan membandingkan password sebagai teks biasa. Ini mudah dipahami untuk latihan lokal, tetapi tidak aman untuk akun nyata atau penggunaan produksi. Sistem produksi harus menyimpan hash password dengan algoritma khusus password, misalnya Argon2.

## 7. Unit test

Di bagian akhir file:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn credential_check_requires_exact_non_empty_password() {
        assert!(super::password_matches("secret", "secret"));
        assert!(!super::password_matches("secret", "wrong"));
        assert!(!super::password_matches("", ""));
    }
}
```

- `#[cfg(test)]` membuat modul test ini dikompilasi saat test dijalankan, bukan sebagai bagian dari aplikasi normal.
- `mod tests` mengelompokkan test untuk modul ini.
- `#[test]` menandai fungsi sebagai unit test.
- `assert!(...)` membuat test gagal jika kondisi di dalamnya bernilai `false`.
- `super::password_matches(...)` memanggil fungsi dari modul satu tingkat di atas (`auth`). Fungsi itu privat, tetapi test berada di dalam modul anak sehingga dapat mengaksesnya.

Test memeriksa tiga keadaan penting: password yang sama diterima, password berbeda ditolak, dan password kosong ditolak.

## Ringkasan alur

```text
Startup ──> initialize_admin_user ──> upsert akun admin di database

Registrasi ──> register_user ──> coba insert akun baru
                                ├─ Ok(true): berhasil dibuat
                                ├─ Ok(false): username sudah dipakai
                                └─ Err: ada masalah database

Login ──> verify_credentials ──> cari password ──> bandingkan
                                ├─ Ok(true): cocok
                                ├─ Ok(false): username/password tidak cocok
                                └─ Err: pemeriksaan mengalami error
```
