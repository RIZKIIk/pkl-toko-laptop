# Catatan Tahap 05 — Listener HTTP

Tahap ini mempertahankan config database, pool koneksi, route, dan handler
dari tahap 04. Listener HTTP dipisahkan menjadi fungsi dalam `server.rs`.

## Alur startup

1. Memuat `.env` dari folder proyek ini.
2. Membaca konfigurasi server dan PostgreSQL.
3. Membuka koneksi database dan menguji query.
4. Membuat router dari modul routes.
5. Menjalankan listener melalui modul server.

CRUD laptop ditambahkan di tahap 06.
