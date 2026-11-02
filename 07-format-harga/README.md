# 07 — Format harga

Tahap ini melanjutkan CRUD PostgreSQL dan mengubah harga angka menjadi teks
Rupiah, misalnya `15000000` menjadi `Rp15.000.000,00`. Format diterapkan
melalui `LaptopResponse`, sehingga data database tetap bertipe angka.
Semua modul koneksi, server, CRUD, model, repository, dan handler dari tahap
06 juga tetap ada.

Salin `.env.example` menjadi `.env`, gunakan nilai `DB_*` database tahap 11,
lalu jalankan `cargo run --manifest-path 07-format-harga/Cargo.toml` dari root.
Tahap ini membaca dan menulis tabel
`public.laptops` yang sama, tanpa membuat atau mengubah skema. Endpoint CRUD
tetap sama:

| Method | Endpoint | Fungsi |
|---|---|---|
| `GET` | `/api/laptops` | Melihat semua laptop |
| `GET` | `/api/laptops/{id}` | Melihat satu laptop |
| `POST` | `/api/laptops` | Menambahkan laptop |
| `PUT` | `/api/laptops/{id}` | Mengganti data laptop |
| `DELETE` | `/api/laptops/{id}` | Menghapus laptop |

Format Rupiah ditempatkan di presenter terpisah. Tahap berikutnya menyusun
respons daftar sebagai JSON array dengan metadata.

Lihat [`catatan/README.md`](catatan/README.md) untuk alur presenter.
