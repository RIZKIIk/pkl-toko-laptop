# `src/config.rs`

> **Navigasi:** [Indeks catatan](../README.md) · [README proyek](../../README.md) · [Endpoint API](../../endpoin.md)

[Buka kode sumber `config.rs`](../../src/config.rs)

## Tanggung jawab file

File ini menjadi satu tempat untuk membaca konfigurasi server dan PostgreSQL. `main.rs` tidak perlu mengetahui cara menyusun opsi SQLx atau nama setiap environment variable; ia cukup meminta `AppConfig` membaca konfigurasi dan membuka pool.

## Bagian kode dan kegunaannya

| Bagian | Tanggung jawab |
| --- | --- |
| `use sqlx::postgres::{PgConnectOptions, PgPoolOptions}` | Membawa dua builder SQLx: satu untuk detail koneksi PostgreSQL, satu untuk pengaturan pool. |
| `use std::{env, error::Error, time::Duration}` | `env` membaca environment variable, `Error` menjadi tipe error umum, dan `Duration` menyatakan batas waktu. |
| `AppConfig` | Menyimpan nilai konfigurasi yang sudah dibaca. |
| `from_env` | Membaca dan mengubah teks environment menjadi `AppConfig`. |
| `connect_database` | Membuat `PgPool` dari konfigurasi database. |
| `initialize_admin_user` | Mendelegasikan seed akun awal ke modul auth. |
| `database_name` | Meminjam nama database untuk ditampilkan pada log startup. |

## Struct `AppConfig`

`AppConfig` menyimpan data berikut.

| Field | Tipe | Dari | Alasan |
| --- | --- | --- | --- |
| `app_host` | `String` | `APP_HOST` | Alamat yang dipakai listener HTTP, misalnya `127.0.0.1`. Bersifat `pub` karena dibaca `main.rs`. |
| `app_port` | `u16` | `APP_PORT` | Port server, misalnya `3000`. `u16` cocok karena port selalu bilangan positif dalam rentang kecil. Bersifat `pub`. |
| `session_cookie_secure` | `bool` | `SESSION_COOKIE_SECURE` | Mengaktifkan atribut `Secure` pada cookie; default `false` untuk HTTP lokal, wajib `true` bila memakai HTTPS. |
| `db_host` | `String` | `DB_HOST` | Host PostgreSQL. Privat agar detail koneksi tetap dikelola file ini. |
| `db_port` | `u16` | `DB_PORT` | Port PostgreSQL. |
| `db_name` | `String` | `DB_NAME` | Nama database. |
| `db_user` | `String` | `DB_USER` | Pengguna PostgreSQL, bukan akun login aplikasi. |
| `db_password` | `String` | `DB_PASSWORD` | Kata sandi koneksi PostgreSQL. |
| `admin_username` | `String` | `ADMIN_USERNAME` | Username akun latihan yang disinkronkan ke tabel users saat startup. |
| `admin_password` | `String` | `ADMIN_PASSWORD` | Password akun latihan yang disimpan langsung di database lokal. Jangan gunakan konfigurasi ini untuk produksi. |

`String` dipakai karena nilai dibaca dari environment dan kemudian dimiliki oleh struct. Field `db_*` tidak diberi `pub`, jadi modul lain tidak bisa mengakses detail koneksi langsung.

## Uraian kode berurutan

1. Dua baris `//!` di awal adalah dokumentasi modul. Baris kedua adalah tautan balik ke catatan ini.
2. Import `PgConnectOptions` dan `PgPoolOptions` membuat file dapat memakai builder SQLx tanpa menulis path yang panjang.
3. `pub struct AppConfig { ... }` mendefinisikan bentuk data konfigurasi. Kata `pub` pada nama struct membuat `main.rs` dapat memakai tipe ini.
4. `impl AppConfig` mengelompokkan fungsi yang bekerja untuk `AppConfig`.
5. `from_env() -> Result<Self, Box<dyn Error + Send + Sync>>` tidak menerima `self` karena ia membuat konfigurasi baru. `Self` pada hasil berarti `AppConfig`.
6. `SESSION_COOKIE_SECURE` dibaca sebagai boolean. Bila tidak ada, nilainya `false` agar cookie dapat dipakai selama pengembangan melalui HTTP lokal.
7. `ADMIN_USERNAME` dan `ADMIN_PASSWORD` wajib tersedia serta tidak boleh kosong. Keduanya dipakai hanya saat seed akun awal; login selanjutnya memverifikasi data pada tabel users.
8. `env::var("APP_PORT")?.parse()?` membaca port sebagai teks, lalu mengubahnya ke `u16`. `?` pertama mengembalikan error bila variabel tidak ada; `?` kedua mengembalikan error bila isinya bukan angka port yang valid.
9. Pemanggilan `env::var` berikutnya membaca konfigurasi database. Tidak ada default, jadi aplikasi berhenti saat salah satunya tidak tersedia.
10. `initialize_admin_user()` mendelegasikan sinkronisasi username dan password akun latihan kepada modul auth setelah migration dijalankan oleh `main.rs`.
11. `connect_database(&self)` meminjam konfigurasi, sehingga isi `AppConfig` tetap dapat dipakai sesudah pool dibuat. Karena membuka koneksi jaringan, fungsi ini `async`.
12. `PgConnectOptions::new()` membuat opsi PostgreSQL kosong. Rangkaian `.host`, `.port`, `.database`, `.username`, dan `.password` mengisi nilai dari field privat.
13. `PgPoolOptions::new()` memulai pengaturan pool. `.max_connections(5)` membatasi maksimal lima koneksi database aktif. `.acquire_timeout(Duration::from_secs(10))` membuat request gagal bila menunggu koneksi pool lebih dari 10 detik.
14. `.connect_with(options).await` mencoba tersambung lalu mengembalikan `PgPool`. Jika gagal, hasilnya `Err(sqlx::Error)` untuk diteruskan ke `main.rs`.
15. `database_name(&self) -> &str` mengembalikan referensi ke teks `db_name`, bukan membuat salinan `String`. Fungsinya hanya dipakai untuk log.

## Bentuk error yang mungkin

- Environment variable wajib tidak ada → `from_env` mengembalikan error.
- `ADMIN_USERNAME` atau `ADMIN_PASSWORD` kosong → konfigurasi ditolak.
- `SESSION_COOKIE_SECURE` bukan boolean → konfigurasi ditolak.
- `APP_PORT` atau `DB_PORT` bukan angka/di luar rentang `u16` → `.parse()` gagal.
- Host, port, kredensial, atau database PostgreSQL salah → `connect_database` mengembalikan `sqlx::Error`.

## Hubungan dengan file lain

`main.rs` memanggil `AppConfig::from_env()`, `connect_database()`, dan `initialize_admin_user()`. Pool diberikan ke router; handler login membaca tabel users dan middleware session memeriksa login sebelum handler laptop. Detail endpoint ada di [routes](routes.md), [auth handler](handlers/auth.md), dan [handlers](handlers.md).
