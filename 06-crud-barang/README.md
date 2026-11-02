# 06 — CRUD barang

Tahap ini menambahkan penyimpanan PostgreSQL dan endpoint CRUD laptop:

| Method | Endpoint | Fungsi |
|---|---|---|
| `GET` | `/api/laptops` | Melihat semua laptop |
| `GET` | `/api/laptops/{id}` | Melihat satu laptop |
| `POST` | `/api/laptops` | Menambahkan laptop |
| `PUT` | `/api/laptops/{id}` | Mengganti data laptop |
| `DELETE` | `/api/laptops/{id}` | Menghapus laptop |

Tahap ini meneruskan konfigurasi, koneksi, route, handler, dan listener dari
tahap 03–05, kemudian menambahkan model, validasi input, error API, dan
repository CRUD.

Salin `.env.example` menjadi `.env`, lalu samakan nilai `DB_*` dengan
konfigurasi PostgreSQL proyek lengkap tahap 11. Tabel yang dipakai adalah
`public.laptops` dari skema tahap 11. Tahap ini tidak membuat tabel atau
menjalankan migration; database harus sudah memiliki skema tersebut.

Jalankan dari root dengan `cargo run --manifest-path
06-crud-barang/Cargo.toml`. Source dipisah menjadi konfigurasi, koneksi
database, server, model, repository, handler, error, dan route. Endpoint
create/update/delete memang mengubah record yang diminta.

Penjelasan lapisan kode tersedia di [`catatan/README.md`](catatan/README.md).
