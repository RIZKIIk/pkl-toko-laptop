# Toko Laptop App

> **Mulai:** [Dokumentasi API](endpoin.md) · [Catatan belajar](catatan/README.md)

REST API inventaris laptop dengan Rust, Axum, SQLx, dan PostgreSQL. Aplikasi menyediakan pemeriksaan layanan, registrasi, login dengan session cookie dan JWT HMAC, serta operasi CRUD laptop.

> [!IMPORTANT]
> Proyek ini dibuat untuk latihan lokal. Password disimpan sebagai teks biasa dan session disimpan di memori; jangan gunakan dengan akun nyata atau untuk produksi.

## Mulai dalam 4 langkah

1. **Siapkan tools:** pasang Rust toolchain dan PostgreSQL.
2. **Atur konfigurasi:** salin `.env.example` menjadi `.env`, lalu isi kredensial database lokal, password admin latihan, dan `JWT_SECRET`.
3. **Buat database:** gunakan nama yang sama dengan nilai `DB_NAME` di `.env`.
4. **Jalankan server:** dari root proyek, jalankan `cargo run`. Migration dan akun admin akan disiapkan saat startup.

Setelah server berjalan, buka [`GET /health`](http://127.0.0.1:3001/health). Jika alamat server berbeda, sesuaikan URL dengan `APP_HOST` dan `APP_PORT`.

### Fitur utama

- CRUD laptop lengkap dengan validasi input dan pengecekan session.
- Autentikasi berbasis cookie session atau bearer JWT HMAC-SHA256 untuk route yang perlu login.
- Response JSON konsisten untuk sukses dan error.
- Struktur modular agar belajar per lapisan lebih mudah dipahami.

| Ingin melakukan… | Buka |
| --- | --- |
| Mencoba request dan melihat contoh response | [Panduan endpoint](endpoin.md) |
| Mengirim request memakai terminal | [Panduan curl](catatan/curl.md) |
| Menjalankan skenario CRUD terurut | [Panduan Postman](catatan/postman.md) |
| Memahami struktur kode dan alur request | [Indeks catatan belajar](catatan/README.md) |

## Struktur proyek

```text
11-toko-laptop-app-lengkap/
├── src/
│   ├── handlers/                 # Menerima request dan menyusun response
│   │   ├── auth.rs               # Registrasi, login, cek session, dan logout
│   │   └── laptops/              # Handler create, read, update, delete
│   ├── models/                   # Entity database dan payload request
│   ├── presenters/               # Bentuk response dan format Rupiah
│   ├── repositories/             # Query PostgreSQL per domain
│   │   ├── laptop_repository.rs
│   │   └── user_repository.rs
│   ├── validation/               # Validasi ID, teks, POST, dan PATCH
│   ├── auth/                     # Seed/verifikasi akun, session, JWT, response
│   │   ├── http.rs
│   │   ├── jwt.rs
│   │   ├── session.rs
│   │   └── mod.rs
│   ├── config.rs                 # Environment dan pool database
│   ├── error.rs                  # Pemetaan error ke HTTP response
│   ├── routes.rs                 # Pemetaan URL dan method ke handler
│   ├── lib.rs                    # Modul library aplikasi
│   └── main.rs                   # Startup server
├── database/migrations/          # SQL untuk membuat tabel
├── catatan/rust/                 # Catatan modular per lapisan dan file
├── postman/                      # Collection dan environment uji API
├── .env.example                  # Template konfigurasi lokal
├── Cargo.toml                    # Manifest Cargo
└── README.md
```

## Persiapan

1. Pasang Rust toolchain dan PostgreSQL.
2. Salin `.env.example` menjadi `.env`, lalu isi nilai koneksi database lokal dan ganti `ADMIN_PASSWORD` dengan password khusus latihan lokal yang tidak digunakan untuk akun lain.
3. Buat database yang namanya sesuai dengan `DB_NAME`.
4. Jalankan server; SQLx menerapkan semua file migration di `database/migrations/` secara otomatis dan mencatatnya di `_sqlx_migrations`.

`.env` berisi konfigurasi lokal dan tidak seharusnya dimasukkan ke version control. `APP_HOST` boleh tidak diisi (default `127.0.0.1`); port aplikasi, seluruh variabel `DB_*`, `ADMIN_USERNAME`, `ADMIN_PASSWORD`, dan `JWT_SECRET` wajib diisi. `JWT_SECRET` harus berisi sedikitnya 32 byte acak; buat dengan `openssl rand -base64 32` dan jangan bagikan atau commit nilainya. Saat startup, aplikasi menyinkronkan akun latihan dari dua variabel admin tersebut. Untuk memudahkan belajar session, password sementara disimpan sebagai teks biasa di database lokal. Ini **tidak aman untuk produksi atau akun nyata**. `SESSION_COOKIE_SECURE` default-nya `false` agar cookie dapat dipakai di HTTP lokal; ubah ke `true` saat server hanya diakses melalui HTTPS.

Tabel `users` memiliki `id`, `username`, `password`, dan `created_at`; tidak ada kolom role.

## Struktur modular

Setiap lapisan memiliki tanggung jawab terpisah: handler mengurus HTTP, model mendefinisikan data, validation memeriksa input, repository menjalankan query, presenter membentuk output, dan auth memisahkan layanan akun, response HTTP, serta session middleware. File `mod.rs` menjadi pintu modul dan mengekspor API yang dipakai lapisan lain.

Catatan mengikuti struktur source di [catatan belajar](catatan/README.md), termasuk halaman khusus untuk setiap submodul autentikasi dan repository user.

Session disimpan di memori dan akan hilang ketika server dimulai ulang. Untuk menjalankan beberapa instance aplikasi, gunakan session store bersama/persisten.

## Menjalankan server

Dari root proyek:

```bash
cargo run
```

Server memakai alamat `APP_HOST:APP_PORT`. Sesuaikan `baseUrl` pada environment Postman jika alamatnya berbeda.

## Endpoint

| Method | Path | Kegunaan |
| --- | --- | --- |
| `GET` | `/health` | Memeriksa server HTTP. |
| `POST` | `/api/auth/register` | Membuat akun baru. |
| `POST` | `/api/auth/login` | Login, membuat cookie session, dan menerbitkan JWT. |
| `GET` | `/api/auth/session` | Melihat pengguna dari session atau JWT. |
| `POST` | `/api/auth/logout` | Menghapus session; JWT tetap berlaku sampai kedaluwarsa. |
| `GET` | `/api/laptops` | Mengambil seluruh laptop (wajib login). |
| `POST` | `/api/laptops` | Membuat laptop (wajib login). |
| `GET` | `/api/laptops/{id}` | Mengambil satu laptop (wajib login). |
| `PATCH` | `/api/laptops/{id}` | Mengubah satu atau beberapa field (wajib login). |
| `DELETE` | `/api/laptops/{id}` | Menghapus laptop (wajib login). |

### Mengakses API dengan `curl`

Daftarkan akun baru. Username maksimal 100 karakter dan harus unik; endpoint mengembalikan `201 Created` jika berhasil atau `409 Conflict` jika sudah digunakan. Registrasi tidak otomatis login. Setelah mendaftar, login memakai akun tersebut:

```bash
curl -i \
  -H 'Content-Type: application/json' \
  -d '{"username":"pengguna-baru","password":"password-lokal"}' \
  http://127.0.0.1:3001/api/auth/register
```

Login dan simpan cookie session ke `cookies.txt`. Response juga berisi `data.access_token`, JWT HMAC-SHA256 yang berlaku satu jam:

```bash
curl -i -c cookies.txt \
  -H 'Content-Type: application/json' \
  -d '{"username":"admin","password":"GANTI_DENGAN_ADMIN_PASSWORD"}' \
  http://127.0.0.1:3001/api/auth/login
```

Autentikasi tetap bisa memakai cookie session seperti sebelumnya:

```bash
curl -i -b cookies.txt http://127.0.0.1:3001/api/laptops
curl -i -b cookies.txt \
  -H 'Content-Type: application/json' \
  -d '{"nama":"ThinkPad E14","merek":"Lenovo","harga":12500000,"stok":5}' \
  http://127.0.0.1:3001/api/laptops
curl -i -b cookies.txt -X PATCH \
  -H 'Content-Type: application/json' \
  -d '{"stok":4}' \
  http://127.0.0.1:3001/api/laptops/1
curl -i -b cookies.txt -X DELETE http://127.0.0.1:3001/api/laptops/1
curl -i -b cookies.txt http://127.0.0.1:3001/api/auth/session
curl -i -b cookies.txt -X POST http://127.0.0.1:3001/api/auth/logout
```

File cookie adalah kredensial session: jangan dibagikan atau di-commit. Logout menghapus session, dan request berikutnya tanpa login mengembalikan `401 Unauthorized`.

Untuk memakai JWT, salin nilai `access_token` dari response login dan kirim sebagai bearer token:

```bash
TOKEN='SALIN_ACCESS_TOKEN_DARI_RESPONSE_LOGIN'
curl -i -H "Authorization: Bearer $TOKEN" http://127.0.0.1:3001/api/laptops
curl -i -H "Authorization: Bearer $TOKEN" http://127.0.0.1:3001/api/auth/session
```

JWT berlaku satu jam. Logout menghapus session cookie, tetapi token JWT yang telah diterbitkan tetap berlaku sampai kedaluwarsa; hapus token dari client setelah logout.

Body POST memerlukan `nama`, `merek`, `harga`, dan `stok`. Body PATCH menerima satu atau beberapa field tersebut; body kosong ditolak. Harga dan stok harus bilangan bulat nol atau lebih.

GET daftar mengembalikan `data` sebagai daftar array. GET detail dan PATCH mengembalikan satu array dengan urutan `[id, nama, merek, harga, stok]`. POST mengembalikan object dengan nama field. Harga pada response diformat sebagai teks Rupiah.

Contoh URL dan response lengkap ada di [panduan endpoint](endpoin.md). Untuk pengujian berurutan gunakan [Postman collection](postman/toko-laptop-app.postman_collection.json) dan [environment](postman/toko-laptop-app.local.postman_environment.json).
Daftar seluruh command curl, termasuk penjelasan `-X`, cookie, header, dan body ada di [panduan curl](catatan/curl.md).

## Alur aplikasi

```text
main
  -> config membaca .env dan membuat PgPool
  -> routes mendaftarkan endpoint, middleware login, dan session store
  -> middleware memeriksa session sebelum handler pada endpoint yang dilindungi
  -> handler mengambil request
       -> validation memeriksa input
       -> repository menjalankan query
       -> presenter membentuk data response
  -> ApiError mengubah kegagalan menjadi HTTP error
```

Mulai dari [catatan belajar](catatan/README.md) untuk penjelasan struktur dan istilah Rust. [Alur request lengkap](catatan/02-alur-kerja-aplikasi.md) dan catatan per lapisan tersedia di folder tersebut.

## Pemeriksaan

Jalankan dari root:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
```
