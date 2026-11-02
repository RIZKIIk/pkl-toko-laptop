# Catatan Tahap 03 — Koneksi database

Tahap ini menjadi fondasi yang dibawa ke semua tahap backend berikutnya.

## Isi modul

- `src/config.rs`: membaca host, port, nama database, dan kredensial dari `.env`.
- `src/database.rs`: membuat pool PostgreSQL serta menjalankan pemeriksaan `SELECT 1`.
- `src/main.rs`: memuat `.env`, membuka koneksi, melaporkan hasil, lalu keluar.

Belum ada router, handler HTTP, tabel yang dibuat, ataupun perubahan record.
Tahap 04 mempertahankan dua modul database ini dan menambahkan server API.
