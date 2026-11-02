# 08 — JSON array

Tahap ini melanjutkan CRUD PostgreSQL, menampilkan harga sebagai teks Rupiah,
dan membentuk response daftar dengan key `data` yang berisi array JSON.
Setiap baris daftar berbentuk `[id, nama, merek, harga, stok]`.
Seluruh modul koneksi, server, CRUD, model, repository, handler, dan presenter
dari tahap sebelumnya tetap berada di folder ini.

Salin `.env.example` menjadi `.env`, gunakan nilai `DB_*` database tahap 11,
lalu jalankan `cargo run --manifest-path 08-json-array/Cargo.toml` dari root.
Tahap ini menggunakan tabel `public.laptops` yang
sama dan tidak menjalankan migration atau DDL. Endpoint CRUD tetap sama:

| Method | Endpoint | Fungsi |
|---|---|---|
| `GET` | `/api/laptops` | Melihat semua laptop |
| `GET` | `/api/laptops/{id}` | Melihat satu laptop |
| `POST` | `/api/laptops` | Menambahkan laptop |
| `PUT` | `/api/laptops/{id}` | Mengganti data laptop |
| `DELETE` | `/api/laptops/{id}` | Menghapus laptop |

Presenter, repository, handler, dan route berada di modul terpisah. Tahap
berikutnya menambahkan login dan logout berbasis session cookie.

Lihat [`catatan/README.md`](catatan/README.md) untuk bentuk response.
