# 10 — Session cookie dan middleware

Tahap ini melindungi endpoint laptop dengan middleware yang memeriksa session
cookie sebelum handler dijalankan. Login membuat session, cookie dikirim ke
client, dan logout menghapus session. Kredensial diverifikasi terhadap akun
yang sudah ada di `public.users`; tahap ini tidak membuat atau menyinkronkan
akun.
Folder ini berisi source modular lengkap tahap 09 beserta modul middleware
dan route yang menambahkan proteksi.

Salin `.env.example` menjadi `.env`, gunakan konfigurasi DB yang sama dengan
tahap 11, lalu jalankan `cargo run --manifest-path
10-session-cookie-middleware/Cargo.toml` dari root. Semua source tahap
sebelumnya ada langsung di `src/`; tidak menggunakan dependency source tahap
09 dan tidak membuat tabel atau menjalankan migration.

| Method | Endpoint | Akses |
|---|---|---|
| `GET` | `/health` | Publik |
| `POST` | `/api/auth/login` | Publik; mengirim kredensial JSON |
| `GET` | `/api/auth/session` | Perlu login |
| `POST` | `/api/auth/logout` | Perlu login |
| Semua | `/api/laptops...` | Perlu login |

Simpan cookie dari response login dan kirim kembali untuk endpoint yang
dilindungi. Session disimpan di memori dan cookie `Secure` dimatikan agar bisa
dipakai pada HTTP lokal. Ini hanya untuk pembelajaran lokal, bukan autentikasi
produksi. Proyek nomor 11 adalah aplikasi lengkap asli dengan validasi,
migration, dan dokumentasi.

Lihat [`catatan/README.md`](catatan/README.md) untuk pembagian route publik
dan route terlindungi.
