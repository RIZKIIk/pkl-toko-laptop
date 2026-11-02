# Catatan Tahap 04 — Routes dan handler

Tahap ini membawa koneksi PostgreSQL tahap 03 ke dalam server HTTP.

## Isi modul

- `config.rs` dan `database.rs`: konfigurasi serta pool PostgreSQL dari tahap 03.
- `handlers.rs`: health check yang menguji koneksi dan handler pengantar laptop.
- `routes.rs`: menghubungkan URL dan method HTTP ke handler.
- `server.rs`: membuka listener HTTP.
- `main.rs`: mengatur urutan startup dan menghubungkan seluruh modul.

Route laptop masih berupa pengantar; operasi CRUD ditambahkan pada tahap 06.
