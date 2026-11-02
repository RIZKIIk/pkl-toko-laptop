# Panduan Presentasi Proyek

> **Navigasi:** [Indeks catatan](README.md) · [README proyek](../README.md) · [Endpoint API](../endpoin.md)

Dokumen ini adalah urutan singkat untuk menjelaskan proyek tanpa harus membuka
semua file sekaligus. Detail tiap fungsi tetap tersedia pada catatan file yang
ditautkan di bawah.

## Ringkasan satu kalimat

Toko Laptop App adalah REST API Rust untuk mengelola inventaris laptop, dengan
registrasi akun dan login berbasis cookie session, PostgreSQL sebagai database,
dan validasi input sebelum query dijalankan.

## Urutan presentasi yang disarankan

1. Mulai dari [README](../README.md) untuk tujuan aplikasi, cara menjalankan,
   dan daftar endpoint.
2. Tunjukkan [struktur proyek](01-gambaran-proyek.md): `routes` menentukan
   URL, `handlers` menangani HTTP, `validation` memeriksa input, `repositories`
   menjalankan SQL, dan `presenters` membentuk response.
3. Jelaskan startup melalui [`main.rs`](rust/main.md): aplikasi membaca `.env`,
   membuat pool PostgreSQL, menjalankan migration, menyiapkan akun admin,
   membuat router, lalu membuka server HTTP.
4. Buka [`routes.rs`](rust/routes.md) untuk menunjukkan pemisahan endpoint
   publik (`/health`, `/api/auth/register`, `/api/auth/login`) dan endpoint
   yang wajib login.
5. Pilih `POST /api/laptops` sebagai contoh alur lengkap: request masuk ke
   [handler create](rust/handlers/laptops/create.md), divalidasi, disimpan oleh
   [repository](rust/repositories/laptop-repository.md), lalu diformat oleh
   [presenter](rust/presenters/laptop-response.md).
6. Jelaskan login memakai [handler auth](rust/handlers/auth.md) dan
   [middleware session](rust/auth/session.md). Session disimpan di memori,
   sehingga logout atau restart server membuat session tidak dapat dipakai.
7. Akhiri dengan [endpoint](../endpoin.md), collection Postman, dan test yang
   dijalankan dengan `cargo test --all-targets`.

## Peta cepat file aktif

```text
main.rs -> config.rs -> PostgreSQL + migration
        -> routes.rs -> middleware session -> handler
                                           -> validation -> repository -> presenter -> JSON
```

## Hal penting untuk disebutkan

- Input laptop diperiksa di aplikasi dan juga dibatasi oleh constraint database.
- Password disimpan sebagai teks biasa hanya untuk pembelajaran session lokal;
  ini bukan rancangan untuk produksi.
- Harga disimpan sebagai angka Rupiah utuh di database, kemudian diubah menjadi
  teks seperti `Rp 12.500.000` ketika dikirim sebagai response.
- Endpoint laptop, pemeriksaan session, dan logout membutuhkan cookie hasil
  login; `/health` dan login sendiri bersifat publik.
