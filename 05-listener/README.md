# 05 — Listener HTTP

Tahap ini melanjutkan koneksi database dan route dari tahap 04, lalu
memisahkan listener TCP ke `src/server.rs`. `APP_HOST` dan `APP_PORT` dibaca
dari `.env`; query health tetap memeriksa koneksi PostgreSQL.

Salin `.env.example` menjadi `.env`, samakan DB dengan konfigurasi tahap 11,
lalu jalankan `cargo run`. Cek server pada `http://127.0.0.1:3005/health`.
Folder `src/` mempertahankan modul `config`, `database`, `handlers`, dan
`routes` dari tahap sebelumnya.

Lihat [`catatan/README.md`](catatan/README.md) untuk alur startup.
