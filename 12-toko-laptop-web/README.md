# LaptopKu

Website toko laptop LaptopKu dengan backend Rust dan frontend web. Struktur proyek dipisah agar source, konfigurasi, database, dan catatan backend mudah ditemukan.

## Struktur proyek

```text
12-toko-laptop-web/
├── backend/       # API Rust, konfigurasi, database, Postman, dan catatan backend
├── frontend/
│   ├── css/       # Style halaman akun, dashboard, komponen, tema, dan responsive
│   └── js/        # Modul API, akun, tema, sidebar, dashboard, dan aksi produk
├── start-containers.sh  # Menyalakan Redis, PostgreSQL, dan RedisInsight
├── flow_toko_laptop.drawio
└── README.md
```

- [Backend README](backend/README.md) — setup PostgreSQL, konfigurasi, endpoint, dan cara menjalankan server.
- [Catatan backend](backend/catatan/README.md) — panduan belajar dari alur aplikasi sampai penjelasan source.
- [Dokumentasi endpoint](backend/endpoin.md) — request, response, dan aturan akses API.
- [Flowchart aplikasi](flow_toko_laptop.drawio)

Di dalam `frontend/js`, `shared.js` menyediakan API dan helper umum. `login.js` dan `register.js` menangani halaman akun, `theme.js` mengatur mode terang/gelap, `sidebar.js` mengatur navigasi yang bisa diciutkan, `dashboard-view.js` membentuk halaman toko, dan `laptop-actions.js` menangani produk serta pembelian.

Backend memakai Redis untuk session. Redis dijalankan dari `backend/compose.yaml`, sedangkan data session dan expiry-nya dikelola oleh `tower-sessions`.

## Menjalankan aplikasi

### Nyalakan PostgreSQL, Redis, dan GUI Redis

Pastikan Docker aktif. Dari root repository, jalankan satu perintah ini untuk menyalakan tiga container yang dipakai:

```bash
./12-toko-laptop-web/start-containers.sh
```

Skrip menyalakan Redis lewat Docker Compose, lalu menyalakan container PostgreSQL `toko-laptop-postgres` dan RedisInsight `pkl-redisinsight`. Skrip juga menampilkan status container. Jika sedang berada di folder `12-toko-laptop-web`, jalankan `./start-containers.sh`.

Perintah manual yang melakukan hal yang sama:

```bash
cd 12-toko-laptop-web/backend
docker compose up -d redis
docker start toko-laptop-postgres pkl-redisinsight
docker ps -a
```

Setelah RedisInsight aktif, buka [http://localhost:5540](http://localhost:5540). Koneksi Redis `127.0.0.1:6379` sudah tersimpan. Jika perlu menambahkannya lagi di RedisInsight, pilih **Add Redis Database**, lalu isi:

| Kolom | Nilai |
| --- | --- |
| Host | `127.0.0.1` |
| Port | `6379` |
| Database | `0` |
| Username | Kosong |
| Password | Kosong |

Container `backend-redis-1` menjalankan Redis untuk session backend, `toko-laptop-postgres` menjalankan database aplikasi, dan `pkl-redisinsight` menyediakan GUI untuk melihat Redis. RedisInsight hanya perlu dibuka ketika ingin melihat atau mengelola isi Redis.

Dari root repository, untuk menghentikan ketiganya:

```bash
docker stop pkl-redisinsight toko-laptop-postgres
cd 12-toko-laptop-web/backend
docker compose stop redis
```

### Jalankan backend

Dari root repository, siapkan database PostgreSQL sesuai `DB_NAME` pada `backend/.env`. Jika file konfigurasi lokal belum ada, salin contoh lalu sesuaikan kredensial:

```bash
cd 12-toko-laptop-web/backend
docker compose up -d redis
cargo run
```

Saat server berjalan, buka [http://127.0.0.1:3001/login](http://127.0.0.1:3001/login). Port mengikuti nilai `APP_PORT` di `backend/.env`.
