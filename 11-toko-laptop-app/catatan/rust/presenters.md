# Presenter response

> **Navigasi:** [Indeks catatan](../README.md) · [README proyek](../../README.md) · [Endpoint API](../../endpoin.md)

[Buka modul presenter](../../src/presenters/mod.rs)

## Tanggung jawab

Presenter memisahkan bentuk model internal dari bentuk JSON yang diterima client. Di lapisan ini harga angka `i64` diformat sebagai teks Rupiah.

## Struktur

```text
presenters/
├── mod.rs
├── laptop_response.rs
└── rupiah.rs
```

| Catatan | Tanggung jawab |
| --- | --- |
| [presenters/mod.rs](presenters/mod.md) | Mengekspor presenter response dan menyimpan formatter sebagai modul internal. |
| [presenters/laptop-response.rs](presenters/laptop-response.md) | Mengubah `Laptop` menjadi array atau object JSON. |
| [presenters/rupiah.rs](presenters/rupiah.md) | Memformat harga dengan awalan `Rp` dan pemisah titik. |

## Format response laptop

- GET daftar, GET detail, dan PATCH menggunakan array `[id, nama, merek, harga, stok]`.
- POST menggunakan object dengan field bernama `id`, `nama`, `merek`, `harga`, dan `stok`.
- Nilai `harga` pada response berupa string Rupiah, walaupun disimpan dan diterima dari request sebagai angka.

Perbedaan bentuk response tersebut adalah kontrak API yang sudah ada, bukan pilihan yang dibuat oleh presenter secara otomatis.
