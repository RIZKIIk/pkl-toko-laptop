# Panduan Postman

> **Navigasi:** [README proyek](../README.md) · [Endpoint API](../endpoin.md) · [Panduan curl](curl.md)

Gunakan collection dan environment lokal berikut untuk mencoba backend:

- [Collection API](../postman/toko-laptop-app.postman_collection.json)
- [Environment lokal](../postman/toko-laptop-app.local.postman_environment.json)

## Persiapan

1. Jalankan PostgreSQL dan server API, lalu import dua file JSON di atas ke Postman.
2. Pilih environment **Toko Laptop App - Local** dan isi `baseUrl`, `adminUsername`, serta `adminPassword`.
3. Atur `registerUsername` dan `registerPassword` di collection variables untuk akun buyer.
4. Jalankan request sesuai urutan. Salin token login admin ke `adminToken`, token login buyer ke `buyerToken`, dan ID laptop dari response tambah ke `laptopId`.

## Urutan request

| Request | Fungsi |
| --- | --- |
| `00 - Register akun` | Membuat akun buyer. Jika sudah pernah dibuat, lanjutkan ke login buyer. |
| `01 - Login admin` | Login admin untuk membuat dan mengubah data laptop. |
| `02 - Daftar laptop` | Menampilkan inventaris. |
| `03 - Tambah laptop` | Membuat laptop contoh dengan stok awal 2. Salin ID hasil response ke `laptopId`. |
| `04 - Detail laptop` | Menampilkan laptop berdasarkan ID. |
| `05 - Ubah laptop` | Mengubah nama dan harga laptop. |
| `06 - Login buyer` | Login memakai akun yang dibuat pada request registrasi. |
| `07 - Beli laptop` | Membeli satu unit dan mengurangi stok. |
| `08 - Hapus laptop` | Menghapus laptop contoh memakai token admin. |

Login mengembalikan `data.access_token`. Simpan token admin dan buyer pada collection variables masing-masing. Cookie session juga dikelola Postman, tetapi collection memakai bearer token agar role admin dan buyer tetap jelas saat dicoba bergantian.

Pada request pembelian, ubah `jumlah` jika ingin membeli lebih dari satu unit. Jumlah harus lebih dari nol dan tidak boleh melebihi stok tersedia. Stok terbaru terlihat di response pembelian atau request daftar/detail laptop.

## Environment

Postman tidak membaca file `.env` backend secara otomatis. Masukkan host, port, dan kredensial admin sendiri di environment lokal. Jangan membagikan file environment yang sudah diisi password atau token.
