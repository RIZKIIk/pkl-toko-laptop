# Endpoint backend (`src/endpoin/`)

> **Navigasi:** [Indeks catatan](../README.md) · [README proyek](../../README.md) · [Endpoint API](../../endpoin.md)

Folder `src/endpoin/` berisi pendaftaran URL dan method HTTP. Setiap file mengelompokkan route berdasarkan fitur, sedangkan handler tetap berada di `src/handlers/`.

## Peta modul

| File | Tanggung jawab |
| --- | --- |
| [`mod.rs`](../../src/endpoin/mod.rs) | Menggabungkan router fitur, memasang middleware autentikasi untuk endpoint privat, lalu memasang state, JWT, dan session. |
| [`login.rs`](../../src/endpoin/login.rs) | `POST /api/auth/login`; endpoint publik. |
| [`register.rs`](../../src/endpoin/register.rs) | `POST /api/auth/register`; endpoint publik. |
| [`session.rs`](../../src/endpoin/session.rs) | `GET /api/auth/session` dan `POST /api/auth/logout`; perlu login. |
| [`laptops.rs`](../../src/endpoin/laptops.rs) | Daftar, detail, tambah, ubah, hapus, dan beli laptop. |
| [`admin.rs`](../../src/endpoin/admin.rs) | Daftar role, membuat akun, dan mengubah role akun. Middleware role membatasi akses ke admin. |
| [`health.rs`](../../src/endpoin/health.rs) | `GET /health`. |
| [`pages.rs`](../../src/endpoin/pages.rs) | Halaman login, register, dashboard, tambah akun, dan redirect dari `/`. |
| [`assets.rs`](../../src/endpoin/assets.rs) | File CSS, JavaScript, dan gambar dari allowlist compile-time. |

`src/routes.rs` hanya menjadi jalur kompatibilitas yang me-re-export `endpoin::buat_router`. Kode server menggunakan `endpoin::buat_router` langsung.

## Cara router dirangkai

`buat_router(...)` di `mod.rs` membangun dua grup:

1. Router publik menggabungkan halaman frontend, aset, health check, login, dan registrasi.
2. Router privat menggabungkan sesi, laptop, serta admin. Middleware `wajibkan_autentikasi` dipasang pada grup ini saja.
3. Router akhir memasang `JwtService`, `SessionManagerLayer` dengan Redis, dan `PgPool` sebagai state bersama.

Dengan susunan ini, route baru cukup ditambahkan pada modul fitur yang sesuai, lalu routernya digabungkan di `mod.rs`. Handler tidak perlu dipindahkan atau dicampur dengan deklarasi URL.

## Peta endpoint

| Method | URL | Modul route | Handler |
| --- | --- | --- | --- |
| GET | `/health` | `health.rs` | `cek_kesehatan` |
| POST | `/api/auth/login` | `login.rs` | `masuk` |
| POST | `/api/auth/register` | `register.rs` | `daftar` |
| GET | `/api/auth/session` | `session.rs` | `pengguna_saat_ini` |
| POST | `/api/auth/logout` | `session.rs` | `keluar` |
| GET / POST | `/api/laptops` | `laptops.rs` | `daftar_laptop` / `tambah_laptop` |
| GET / PATCH / DELETE | `/api/laptops/{id}` | `laptops.rs` | `ambil_laptop` / `ubah_laptop` / `hapus_laptop` |
| POST | `/api/laptops/{id}/buy` | `laptops.rs` | `beli_laptop` |
| GET | `/api/admin/roles` | `admin.rs` | `daftar_peran_admin` |
| POST | `/api/admin/users` | `admin.rs` | `buat_pengguna` |
| PATCH | `/api/admin/users/{username}/role` | `admin.rs` | `ubah_peran_pengguna` |
| GET | `/`, `/login`, `/register`, `/dashboard`, `/dashboard/users/new` | `pages.rs` | Halaman frontend / redirect |
| GET | `/assets/{*path}` | `assets.rs` | Allowlist aset frontend |

## Hubungan dengan file lain

- [`main.rs`](main.md) memanggil `endpoin::buat_router` setelah konfigurasi, database, dan Redis siap.
- [`handlers/`](handlers.md) menjalankan alur request yang ditunjuk oleh route.
- [`auth/middleware.rs`](auth/middleware.md) memeriksa identitas dan izin role pada grup privat.
- [Dokumentasi endpoint API](../../endpoin.md) menjelaskan payload dan response untuk client.
