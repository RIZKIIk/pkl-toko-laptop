# Catatan Belajar Toko Laptop App

> **Navigasi:** [README proyek](../README.md) · [Dokumentasi API](../endpoin.md)

Dokumentasi ini menjelaskan struktur aplikasi Rust di root, alur request, serta peran file source. Kode dikelompokkan per lapisan, lalu file dipisah berdasarkan tanggung jawabnya.

> [!TIP]
> Baru mengenal proyek ini? Ikuti **Urutan belajar** dari nomor 1. Setiap halaman menyediakan tautan kembali ke indeks, README proyek, dan dokumentasi endpoint.

## Gambaran cepat

- `src/routes.rs` menentukan URL dan middleware autentikasi.
- `src/handlers/` menerima request HTTP dan mengirim response.
- `src/validation/` memeriksa input sebelum query database.
- `src/repositories/` mengelola query PostgreSQL per domain.
- `src/presenters/` mengubah model menjadi JSON yang siap dikirim client.

## Cara membaca catatan

Setiap halaman kode mengikuti urutan source: gambaran tanggung jawab, import
atau deklarasi modul, lalu fungsi dan alur penting dari atas ke bawah. Istilah
Rust yang muncul dijelaskan saat pertama kali dipakai, dan contoh response
menunjukkan bentuk data yang diterima client.

- **Buka source** membawa kembali ke file Rust yang sedang dijelaskan.
- **Navigasi** membawa ke indeks, README proyek, atau daftar endpoint.
- Potongan kode yang diawali `let` atau contoh pemanggilan adalah ilustrasi
  untuk membantu memahami konsep; ikuti catatan di sekitarnya untuk konteks.
- Nama seperti `StatusCode::OK` atau `Option<T>` adalah sintaks Rust, bukan teks
  yang harus diketik sebagai bagian dari URL.

Jika baru belajar, mulai dari [gambaran proyek](01-gambaran-proyek.md), lalu
ikuti [alur kerja aplikasi](02-alur-kerja-aplikasi.md). Gunakan daftar di bawah
untuk membuka detail file saat ada istilah atau langkah yang belum jelas.

## Urutan belajar

1. [Panduan presentasi](00-panduan-presentasi.md) memberi urutan singkat untuk menjelaskan proyek.
2. [Gambaran proyek](01-gambaran-proyek.md) memperkenalkan teknologi dan lapisan aplikasi.
3. [Alur kerja aplikasi](02-alur-kerja-aplikasi.md) mengikuti startup dan request sampai database.
4. [Startup server](rust/main.md), [config](rust/config.md), dan [lib](rust/lib.md) menjelaskan persiapan aplikasi.
5. [Routes](rust/routes.md) menunjukkan pemetaan URL ke handler dan middleware session/JWT.
6. [Handlers](rust/handlers.md) menjelaskan alur request dengan tautan ke catatan tiap operasi.
7. Pelajari [autentikasi modular](rust/auth/mod.md), [models](rust/models.md), [validation](rust/validation.md), [repositories](rust/repositories.md), dan [presenters](rust/presenters.md).
8. Lihat [error API](rust/error.md), seluruh [migration database](rust/database/0001-create-laptops.md), dan [dependensi](rust/cargo-toml.md).

## Dokumentasi per file

- **Handler:** [auth](rust/handlers/auth.md), [system](rust/handlers/system.md), [create](rust/handlers/laptops/create.md), [read](rust/handlers/laptops/read.md), [update](rust/handlers/laptops/update.md), dan [delete](rust/handlers/laptops/delete.md).
- **Model:** [entity](rust/models/laptop/entity.md), [payload create](rust/models/laptop/create-payload.md), dan [payload update](rust/models/laptop/update-payload.md).
- **Validation:** [ID](rust/validation/id.md), [teks](rust/validation/text.md), [create](rust/validation/create.md), dan [update](rust/validation/update.md).
- **Repository:** [modul repository](rust/repositories/mod.md), [query laptop](rust/repositories/laptop-repository.md), dan [query user](rust/repositories/user-repository.md).
- **Autentikasi:** [modul auth](rust/auth/mod.md), [session](rust/auth/session.md), [JWT](rust/auth/jwt.md), [response auth](rust/auth/http.md), dan [query user](rust/repositories/user-repository.md).
- **Database:** [migration laptop](rust/database/0001-create-laptops.md), [skema awal users](rust/database/0002-create-users.md), dan [skema password latihan](rust/database/0003-plaintext-password.md).
- **Presenter:** [response laptop](rust/presenters/laptop-response.md) dan [format Rupiah](rust/presenters/rupiah.md).

## Mencoba endpoint

Daftar endpoint dan response tersedia di [endpoin.md](../endpoin.md). Contoh command lengkap beserta penjelasan opsi curl ada di [panduan curl](curl.md). Collection dan environment Postman ada di [postman/](../postman/toko-laptop-app.postman_collection.json) dan file environment JSON di folder yang sama.

Komentar dokumentasi pada source Rust menyediakan tautan `file://` langsung ke catatan terkait. Dari catatan, tautan “Buka source” kembali ke implementasi.
