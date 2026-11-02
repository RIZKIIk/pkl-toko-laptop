# 09 — Login dan logout

Tahap ini menambahkan login dan logout menggunakan session cookie. Login
memeriksa `username` dan `password` pada tabel `public.users`, sama seperti
skema versi lengkap tahap 11. Session disimpan di memori server. Endpoint
laptop pada tahap ini belum dilindungi middleware; proteksi ditambahkan pada
tahap 10.
Modul koneksi, server, CRUD, presenter, route, dan handler laptop dari tahap
sebelumnya tetap disertakan.

Salin `.env.example` menjadi `.env`, samakan nilai `DB_*` dengan konfigurasi
tahap 11, lalu jalankan `cargo run --manifest-path 09-login-logout/Cargo.toml`
dari root. Tahap ini tidak menjalankan migration
atau menyinkronkan akun. Gunakan akun yang sudah tersedia pada tabel
`public.users`; versi lengkap mengelola akun latihan dari konfigurasi admin.

| Method | Endpoint | Fungsi |
|---|---|---|
| `POST` | `/api/auth/login` | Login dan menerima session cookie |
| `GET` | `/api/auth/session` | Melihat identitas pada session aktif |
| `POST` | `/api/auth/logout` | Menghapus session aktif |

Simpan cookie dari response login dan kirim kembali saat memeriksa session atau
logout. Session di memori akan hilang saat server dimulai ulang. Fitur ini
hanya untuk pembelajaran lokal, bukan autentikasi produksi. Source dipisah
antara handler autentikasi/laptop, repository, presenter, konfigurasi, dan
route. Tahap 10 menambahkan middleware untuk mewajibkan login.

Lihat [`catatan/README.md`](catatan/README.md) untuk modul autentikasi.
