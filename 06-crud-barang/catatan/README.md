# Catatan Tahap 06 — CRUD barang

Tahap ini mempertahankan konfigurasi, koneksi database, route, handler, dan
listener. Fitur baru dipisah ke lapisan domain agar setiap file punya tugas
yang jelas.

## Lapisan aplikasi

- `models.rs`: entity laptop dan validasi dasar payload.
- `repository.rs`: query CRUD ke `public.laptops`.
- `handlers.rs`: mengubah request HTTP menjadi panggilan repository.
- `error.rs`: memetakan kesalahan database, input, dan data tidak ditemukan.
- `routes.rs`: mendaftarkan endpoint koleksi dan endpoint berdasarkan ID.

Tabel harus sudah tersedia dari skema tahap 11; tahap ini tidak menjalankan
migration atau DDL.
