<div align="center">

# 💻 Proyek Toko Laptop

**Rust Learning Journey · Backend Development · PostgreSQL**

Kumpulan proyek pembelajaran Rust untuk membangun aplikasi toko laptop secara bertahap, mulai dari program terminal sederhana hingga REST API dengan database, autentikasi, dan session.

<br>

![Rust](https://img.shields.io/badge/Rust-Programming%20Language-orange?logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-Database-4169E1?logo=postgresql&logoColor=white)
![API](https://img.shields.io/badge/REST-API-009688)
![Learning](https://img.shields.io/badge/Project-Learning-blue)

</div>

---

## 📖 Tentang Proyek

Repository ini berisi kumpulan proyek Cargo yang dibuat untuk mempelajari pengembangan aplikasi menggunakan **Rust** secara bertahap.

Setiap tahap memperkenalkan konsep dan fitur baru. Materi dimulai dari dasar pemrograman, pengelolaan data menggunakan `struct` dan `Vec`, hingga pengembangan backend menggunakan Axum dan PostgreSQL.

### 🎯 Tujuan Pembelajaran

- Memahami fundamental bahasa pemrograman Rust.
- Mengenal struktur dan modularisasi proyek.
- Memahami konsep CRUD dan pengelolaan database.
- Membangun REST API menggunakan Rust.
- Mengimplementasikan autentikasi dan session.
- Memahami middleware dan pemisahan tanggung jawab kode.

---

## 🗂️ Daftar Tahapan Proyek

Setiap folder bernomor merupakan proyek Cargo tersendiri.

Tahap **03–11** merupakan pengembangan backend API yang berada di root proyek ini. Tahap **03–10** bersifat kumulatif, artinya source code dan fitur dari tahap sebelumnya tetap dipertahankan ketika memasuki tahap berikutnya.

Tahap **11** merupakan aplikasi final yang menggabungkan fitur-fitur utama.

| Tahap | Folder | Materi & Fitur |
|:---:|---|---|
| 01 | [`01.toko_laptop/`](01.toko_laptop/) | **Terminal Application** — Program toko laptop sederhana melalui terminal. Pengguna dapat memilih laptop, memasukkan nama, melihat total belanja, dan mendapatkan struk. Mempelajari input, kondisi, dan perulangan. |
| 02 | [`02.toko_laptop_crud_memori/`](02.toko_laptop_crud_memori/) | **CRUD In-Memory** — Mengelola data laptop menggunakan `struct` dan `Vec`. Mendukung tambah, lihat, ubah, dan hapus data. Data belum tersimpan secara permanen. |
| 03 | [`03-koneksi-database/`](03-koneksi-database/) | **Database Connection** — Mengenal konfigurasi aplikasi, koneksi PostgreSQL, dan pengujian koneksi menggunakan `SELECT 1`. |
| 04 | [`04-routes-handler/`](04-routes-handler/) | **Routes & Handlers** — Menambahkan routing, handler, health check database, dan endpoint pengantar untuk data laptop. |
| 05 | [`05-listener/`](05-listener/) | **HTTP Server** — Memisahkan listener HTTP ke dalam modul server dan menambahkan konfigurasi alamat server. |
| 06 | [`06-crud-barang/`](06-crud-barang/) | **Database CRUD** — Implementasi CRUD PostgreSQL dengan model, validasi, error handling, dan repository. |
| 07 | [`07-format-harga/`](07-format-harga/) | **Currency Formatter** — Menambahkan presenter untuk mengubah nominal harga menjadi format Rupiah. |
| 08 | [`08-json-array/`](08-json-array/) | **JSON Response** — Mengembangkan format response daftar barang menjadi JSON array. |
| 09 | [`09-login-logout/`](09-login-logout/) | **Authentication** — Menambahkan fitur login, logout, dan pengelolaan session menggunakan cookie. |
| 10 | [`10-session-cookie-middleware/`](10-session-cookie-middleware/) | **Middleware** — Mengembangkan middleware untuk memeriksa autentikasi dan melindungi endpoint tertentu. |
| 11 | [`11-toko-laptop-app/`](11-toko-laptop-app/) | **Final Application** — REST API lengkap dengan CRUD, autentikasi, validasi, presenter, repository, migration, dokumentasi, dan koleksi Postman. |

---

## 🏗️ Alur Pengembangan

Proyek ini dikembangkan melalui beberapa tahapan utama:

```text
01. Terminal Application
          │
          ▼
02. In-Memory CRUD
          │
          ▼
03. Database Connection
          │
          ▼
04. Routes & Handlers
          │
          ▼
05. HTTP Server
          │
          ▼
06. PostgreSQL CRUD
          │
          ▼
07. Currency Formatter
          │
          ▼
08. JSON Array Response
          │
          ▼
09. Login & Logout
          │
          ▼
10. Session & Middleware
          │
          ▼
11. FINAL APPLICATION
```

---

## 📚 Dokumentasi

Dokumentasi lebih lengkap mengenai aplikasi final tersedia di folder tahap 11.

**[📂 Buka Dokumentasi Tahap 11](11-toko-laptop-app/README.md)**

Dokumentasi tersebut mencakup:

- Arsitektur dan struktur proyek.
- Penjelasan modul dan fungsi kode.
- Daftar endpoint REST API.
- Contoh penggunaan `curl`.
- Konfigurasi database dan migration.
- Panduan pengujian menggunakan Postman.

---

<div align="center">

**Built with Rust 🦀**

*Learning by Building, Improving Step by Step.*

</div>