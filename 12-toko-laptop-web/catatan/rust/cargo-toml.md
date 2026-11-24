# `Cargo.toml`

> **Navigasi:** [Indeks catatan](../README.md) · [README proyek](../../README.md) · [Endpoint API](../../endpoin.md)

![Ilustrasi: Peta dependency Cargo](../ilustrasi/halaman/11-cargo-toml.svg "Peta dependency Cargo")


[Buka file `Cargo.toml`](../../Cargo.toml)

## Tanggung jawab file

`Cargo.toml` adalah manifest Cargo di root proyek: file yang memberi nama package, memilih edisi Rust, dan mencatat library yang perlu diunduh/dikompilasi. File ini tidak dijalankan saat request API datang, tetapi tanpa file ini Cargo tidak tahu cara membangun aplikasi.

## Peta dependency

```text
Cargo.toml
├── runtime/server : tokio + axum
├── database       : sqlx (PostgreSQL + migration)
├── data JSON      : serde + serde_json
├── auth           : tower-sessions + jsonwebtoken + argon2
└── konfigurasi    : dotenvy
```

`Cargo.lock` kemudian mengunci versi dependency yang benar-benar dipakai dalam build proyek ini.

## Bagian `[package]`

| Baris | Arti |
| --- | --- |
| `default-run = "toko_laptop_app"` | `cargo run` tanpa `--bin` memilih binary dengan nama ini. |
| `name = "toko_laptop_app"` | Nama package/crate. Nama ini dipakai oleh `main.rs` saat `use toko_laptop_app::{...}`. |
| `version = "0.1.0"` | Nomor versi package saat ini. Tidak mengubah perilaku API. |
| `edition = "2024"` | Memilih aturan bahasa Rust edisi 2024 untuk kompilasi. |

## Bagian `[dependencies]`

| Dependency | Versi/fitur | Dipakai di kode untuk |
| --- | --- | --- |
| `axum = "0.8.9"` | Versi 0.8.9 | `Router`, route, extractor `Json`/`Path`/`State`, response, dan menjalankan server. |
| `tokio` | `macros`, `rt-multi-thread`, `net` | `#[tokio::main]`, runtime async multi-thread, dan `TcpListener`. |
| `sqlx` | versi `0.9.0`; `default-features = false`; `runtime-tokio`, `tls-rustls`, `postgres`, `macros`, `migrate` | Pool PostgreSQL, query, derive `FromRow` untuk entity laptop, migrasi database, dan runtime koneksi. |
| `dotenvy = "0.15"` | Versi 0.15 | Membaca `.env` pada startup. |
| `serde` | versi `1`, fitur `derive` | Membuat implementasi `Deserialize` pada payload JSON dan `Serialize` pada claims JWT/role. |
| `serde_json = "1"` | Versi major 1 | `Value` dan macro `json!` untuk response API. |
| `tower-sessions = "0.15.0"` | Store memory dan middleware Axum | Cookie session serta penyimpanan session server-side. |
| `jsonwebtoken = "9.3.1"` | JWT dengan dukungan HMAC | Menerbitkan dan memverifikasi access token HS256. |
| `argon2 = "0.5.3"` | fitur `std` dan fitur default Argon2id | Hash dan verifikasi password. |
| `rand_core = "0.6"` | fitur `getrandom` | Membuat salt acak yang aman untuk setiap hash password. |
| `tower` | dev dependency, fitur `util` | `ServiceExt::oneshot` untuk tes router HTTP. |

### Mengapa fitur SQLx ditulis khusus

`default-features = false` mematikan fitur bawaan SQLx yang tidak diperlukan. Daftar `features` lalu memilih kemampuan yang memang dipakai:

- `runtime-tokio`: SQLx berjalan pada runtime Tokio.
- `tls-rustls`: dukungan TLS memakai Rustls.
- `postgres`: mengaktifkan tipe dan driver PostgreSQL seperti `PgPool`.
- `macros`: mengaktifkan macro/derive SQLx, termasuk `FromRow` yang dipakai model.
- `migrate`: mengaktifkan macro migrasi yang menerapkan file di `database/migrations/` saat startup.

Autentikasi menerima session cookie atau JWT bearer HS256. Password di-hash menggunakan Argon2id; `tower` hanya diperlukan untuk tes. `Cargo.lock` menyimpan versi tepat dependency yang dipilih Cargo.

## Membaca sintaks manifest

- `[package]`, `[dependencies]`, dan `[dev-dependencies]` adalah tabel TOML. Package berisi metadata; dependencies diperlukan untuk build normal; dev-dependencies hanya dibutuhkan alat pengujian.
- `name`, `version`, `edition`, dan `default-run` adalah key manifest, bukan kode Rust. Versi package (`0.1.0`) berbeda dari versi crate dependency.
- `"1"` pada serde/serde_json adalah persyaratan versi semver untuk major 1, bukan selalu berarti Cargo mengunduh versi tepat 1.0. `Cargo.lock` mencatat resolusi tepat untuk aplikasi.
- Bentuk inline seperti `{ version = "1", features = [...] }` memberi opsi tambahan. Daftar `features` mengaktifkan kemampuan crate secara selektif; fitur yang dipilih harus sesuai dengan API yang dipakai kode.
- `#[derive(...)]` adalah sintaks attribute Rust pada source, bukan bagian manifest. Manifest menyediakan fitur crate yang memungkinkan derive/macro terkait.
