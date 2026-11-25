# Toko Laptop

Aplikasi inventaris dan pembelian laptop dengan backend Rust dan frontend web. Struktur proyek dipisah agar kode, konfigurasi, database, dan catatan backend mudah ditemukan.

## Struktur proyek

```text
12-toko-laptop-web/
├── backend/       # API Rust, konfigurasi, database, Postman, dan catatan backend
├── frontend/
│   ├── css/       # Style dasar, login, dashboard, komponen, dan responsive
│   └── js/        # Modul API, login, dashboard, dan aksi laptop
├── flow_toko_laptop.drawio
└── README.md
```

- [Backend README](backend/README.md) — setup PostgreSQL, konfigurasi, endpoint, dan cara menjalankan server.
- [Catatan backend](backend/catatan/README.md) — panduan belajar dari alur aplikasi sampai penjelasan source.
- [Dokumentasi endpoint](backend/endpoin.md) — request, response, dan aturan akses API.
- [Flowchart aplikasi](flow_toko_laptop.drawio)

Di dalam `frontend/js`, `shared.js` berisi API dan helper umum, `login.js` mengatur form login, `dashboard-view.js` membentuk tampilan inventaris, `dashboard.js` memuat data terproteksi, `laptop-actions.js` menangani CRUD/pembelian, dan `app.js` menghubungkan semuanya.

Backend memakai Redis untuk session. Redis dijalankan dari `backend/compose.yaml`, sedangkan data session dan expiry-nya dikelola oleh `tower-sessions`.

## Menjalankan aplikasi

Siapkan PostgreSQL dan buat database sesuai `DB_NAME` pada `backend/.env`. Jika file konfigurasi lokal belum ada, salin contoh lalu sesuaikan kredensial:

```bash
cd backend
cp -n .env.example .env
docker compose up -d redis
cargo run
```

Saat server berjalan, buka [http://127.0.0.1:3001/login](http://127.0.0.1:3001/login). Port mengikuti nilai `APP_PORT` di `backend/.env`.
