# 04 — Routes dan handler

Tahap ini mempertahankan koneksi PostgreSQL dari tahap 03, kemudian
menambahkan router Axum dan handler. `GET /health` memeriksa database,
sedangkan `GET /api/laptops` menjadi pengantar endpoint sebelum CRUD tahap 06.
Modul koneksi, konfigurasi, route, dan handler tetap berada dalam proyek ini.

Salin `.env.example` menjadi `.env`, samakan nilai DB dengan tahap 11, lalu
jalankan dari root:

```bash
cargo run --manifest-path 04-routes-handler/Cargo.toml
```

Port default: `3004`.

Catatan susunan modul tersedia di [`catatan/README.md`](catatan/README.md).
