# Proyek Toko Laptop

Kumpulan proyek Rust untuk belajar membuat aplikasi toko laptop secara
bertahap, mulai dari program terminal sederhana sampai REST API dengan
PostgreSQL, autentikasi, dan session.

Setiap folder bernomor merupakan proyek Cargo tersendiri. Tahap backend API
03–11 berada langsung di root proyek ini. Tahap 03–10 bersifat kumulatif:
source modular dari kemampuan tahap sebelumnya tetap disertakan, lalu tahap
berikutnya menambah fitur baru. Tahap 11 adalah aplikasi lengkap final.

## Daftar folder dan fungsi

| Tahap | Folder | Fungsi dan materi |
| --- | --- | --- |
| 01 | [`01.toko_laptop/`](01.toko_laptop/) | Program toko melalui terminal. Pengguna memilih laptop, memasukkan nama, lalu melihat total belanja dan struk sederhana. Mengenalkan input, kondisi, dan perulangan. |
| 02 | [`02.toko_laptop_crud_memori/`](02.toko_laptop_crud_memori/) | CRUD laptop di memori menggunakan `struct` dan `Vec`. Data dapat ditambah, dilihat, diubah, dan dihapus, tetapi hilang saat program ditutup. |
| 03 | [`03-koneksi-database/`](03-koneksi-database/) | Proyek awal backend: konfigurasi dan koneksi PostgreSQL, lalu uji `SELECT 1`. |
| 04 | [`04-routes-handler/`](04-routes-handler/) | Mempertahankan koneksi tahap 03; menambah route, handler, health check database, dan endpoint laptop pengantar. |
| 05 | [`05-listener/`](05-listener/) | Mempertahankan modul tahap 04; merapikan listener HTTP ke modul server dan menambah konfigurasi alamat. |
| 06 | [`06-crud-barang/`](06-crud-barang/) | Mempertahankan koneksi/server; menambah model, validasi, error, repository, dan CRUD PostgreSQL. |
| 07 | [`07-format-harga/`](07-format-harga/) | Mempertahankan CRUD tahap 06; menambah presenter format Rupiah. |
| 08 | [`08-json-array/`](08-json-array/) | Mempertahankan modul tahap 07; menambah format daftar response JSON array. |
| 09 | [`09-login-logout/`](09-login-logout/) | Mempertahankan fitur API sebelumnya; menambah login, logout, dan session cookie. |
| 10 | [`10-session-cookie-middleware/`](10-session-cookie-middleware/) | Mempertahankan source modular tahap 09; menambah middleware pelindung endpoint. |
| 11 | [`11-toko-laptop-app-lengkap/`](11-toko-laptop-app-lengkap/) | Aplikasi REST API lengkap: CRUD, autentikasi, validasi, presenter, repository, migration, dokumentasi belajar, dan koleksi Postman. |

## Dokumentasi tambahan

Dokumentasi arsitektur, endpoint, contoh `curl`, dan Postman untuk aplikasi
lengkap tersedia di [`11-toko-laptop-app-lengkap/`](11-toko-laptop-app-lengkap/README.md).
