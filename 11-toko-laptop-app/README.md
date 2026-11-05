# Toko Laptop App

REST API inventaris laptop yang dibuat dengan Rust, Axum, SQLx, dan PostgreSQL. Proyek ini cocok untuk belajar alur request API dari route sampai query database.

## Mulai cepat

### Yang diperlukan

- Rust toolchain (edition 2024).
- PostgreSQL yang sudah berjalan.
- Database PostgreSQL untuk aplikasi; database harus kosong jika baru pertama kali disiapkan.

### Siapkan konfigurasi

Salin `.env.example` menjadi `.env`, lalu sesuaikan nilainya dengan PostgreSQL lokal. Buat database dengan nama yang sama seperti `DB_NAME`.

| Variabel | Wajib | Kegunaan |
| --- | --- | --- |
| `APP_HOST` | Tidak | Alamat bind server; default `127.0.0.1`. |
| `APP_PORT` | Ya | Port HTTP, misalnya `3001`. |
| `SESSION_COOKIE_SECURE` | Tidak | `true` jika cookie hanya dikirim melalui HTTPS; default `false`. |
| `JWT_SECRET` | Ya | Secret HMAC minimal 32 byte untuk menandatangani JWT. |
| `DB_HOST`, `DB_PORT` | Ya | Alamat dan port PostgreSQL. |
| `DB_NAME`, `DB_USER`, `DB_PASSWORD` | Ya | Database dan kredensial koneksi PostgreSQL. |
| `ADMIN_USERNAME`, `ADMIN_PASSWORD` | Ya | Akun latihan yang disinkronkan ketika server mulai. |

Buat secret acak, misalnya dengan `openssl rand -base64 32`. Jangan membagikan `.env`, JWT, atau file cookie session. File `.env` sudah diabaikan oleh Git.

### Jalankan server

```bash
cargo run
```

Migration diterapkan otomatis saat startup. Password lama yang masih tersimpan sebagai teks biasa akan di-hash dengan Argon2id lalu nilai lamanya dihapus. Aplikasi juga membuat atau memperbarui akun admin berdasarkan konfigurasi. Jika server berhasil dimulai, periksa [http://127.0.0.1:3001/health](http://127.0.0.1:3001/health) atau ubah alamat sesuai `APP_HOST` dan `APP_PORT`.

Jangan mengubah file migration yang sudah pernah dijalankan. SQLx memeriksa checksum-nya dan menghentikan startup jika isi file berbeda; buat file migration baru untuk perubahan skema berikutnya.

## Coba API

Login melalui Postman atau `curl` untuk memperoleh session cookie dan JWT. Operasi laptop menerima salah satu dari keduanya.

| Method | Endpoint | Kegunaan | Akses |
| --- | --- | --- | --- |
| `GET` | `/health` | Memeriksa server HTTP. | Publik |
| `POST` | `/api/auth/register` | Mendaftarkan akun. | Publik |
| `POST` | `/api/auth/login` | Login dan menerbitkan session serta JWT. | Publik |
| `GET` | `/api/auth/session` | Melihat identitas pengguna aktif. | Login |
| `POST` | `/api/auth/logout` | Mencabut semua session dan JWT milik pengguna. | Login |
| `POST` | `/api/admin/users` | Membuat akun buyer/restocker. | Admin |
| `PATCH` | `/api/admin/users/{username}/role` | Mengubah role menjadi buyer/restocker. | Admin |
| `GET` | `/api/laptops` | Melihat daftar laptop. | Admin, buyer, restocker |
| `POST` | `/api/laptops` | Menambahkan laptop. | Admin |
| `GET` | `/api/laptops/{id}` | Melihat detail laptop. | Admin, buyer, restocker |
| `PATCH` | `/api/laptops/{id}` | Mengubah field tertentu. | Admin, restocker |
| `DELETE` | `/api/laptops/{id}` | Menghapus laptop. | Admin |

- Lihat body request, response, validasi, dan status error di [dokumentasi endpoint](endpoin.md).
- Salin perintah terminal dari [panduan curl](catatan/curl.md).
- Jalankan skenario terurut dari [panduan Postman](catatan/postman.md) memakai collection dan environment di folder `postman/`.

Login memberi JWT yang berlaku satu jam. Logout mencabut JWT yang dipakai pada request itu dan menghapus session aktif. Token lain milik pengguna tidak ikut dicabut; client tetap perlu menghapus token lokal setelah selesai.

Password akun disimpan sebagai hash Argon2id dengan salt acak, bukan sebagai teks asli. Setelah update pertama kali, migration dan proses startup meng-upgrade password lama otomatis; akun yang sudah terdaftar tetap bisa login dengan password yang sama.

Pendaftaran publik selalu membuat role `buyer`. Akun admin disinkronkan dari `ADMIN_USERNAME` dan `ADMIN_PASSWORD`; admin dapat membuat akun buyer/restocker atau mengubah role lewat endpoint admin. Buyer hanya dapat melihat laptop, restocker dapat melihat serta memperbarui laptop, dan admin dapat menjalankan semua operasi.

## Cara kerja aplikasi

```text
Request HTTP
    ↓
Route dan middleware autentikasi
    ↓
Handler → Validation → Repository → PostgreSQL
    ↓
Presenter → Response JSON
```

| Bagian | Tugas |
| --- | --- |
| `src/main.rs` | Menyiapkan konfigurasi, database, migration, router, dan server. |
| `src/routes.rs` | Menghubungkan URL dan method ke handler serta middleware. |
| `src/handlers/` | Mengatur alur request dan response per endpoint. |
| `src/validation/` | Memeriksa dan membersihkan input sebelum query. |
| `src/repositories/` | Menjalankan query PostgreSQL. |
| `src/models/` | Mendefinisikan entity dan payload JSON. |
| `src/presenters/` | Menyusun bentuk response, termasuk format Rupiah. |
| `src/auth/` | Mengelola akun, session, JWT, dan response autentikasi. |
| `src/error.rs` | Mengubah error aplikasi menjadi response JSON. |
| `database/migrations/` | Menyimpan perubahan skema database. |
| `catatan/` | Panduan belajar per alur, lapisan, dan file source. |

Untuk memahami proyek atau menyiapkan presentasi, mulai dari [indeks catatan belajar](catatan/README.md), lalu ikuti [gambaran proyek](catatan/01-gambaran-proyek.md) dan [alur kerja aplikasi](catatan/02-alur-kerja-aplikasi.md). Ada juga [panduan presentasi](catatan/00-panduan-presentasi.md) singkat.

## Aturan data utama

- Nama laptop wajib diisi dan dibatasi hingga 150 karakter; merek dibatasi hingga 100 karakter.
- Harga adalah bilangan bulat Rupiah dan stok adalah bilangan bulat; keduanya tidak boleh negatif.
- Request `PATCH` boleh mengirim sebagian field, tetapi minimal satu field harus disertakan.
- Nilai harga disimpan sebagai angka di database dan ditampilkan sebagai teks, misalnya `Rp 12.500.000`.
- Daftar, detail, dan update mengembalikan laptop sebagai array berurutan `[id, nama, merek, harga, stok]`; create mengembalikan object bernama.

## Pemeriksaan lokal

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
```
