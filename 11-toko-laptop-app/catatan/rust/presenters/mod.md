# `src/presenters/mod.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

![Ilustrasi: Modul presenter mengekspos formatter](../../ilustrasi/halaman/35-mod.svg "Modul presenter mengekspos formatter")


[Buka source `presenters/mod.rs`](../../../src/presenters/mod.rs)

## Tujuan

File ini menyusun modul presenter. Presenter mengubah entity internal menjadi nilai yang akan dikirim sebagai JSON, tetapi tidak memilih status HTTP dan tidak menjalankan query database.

## Deklarasi modul

```rust
pub mod laptop_response;

mod rupiah;
```

| Deklarasi | Lokasi implementasi | Visibilitas | Kegunaan |
| --- | --- | --- | --- |
| `pub mod laptop_response;` | `laptop_response.rs` | Bisa diakses handler melalui `presenters::laptop_response`. | Mengubah `Laptop` menjadi array atau object JSON. |
| `mod rupiah;` | `rupiah.rs` | Detail internal modul presenter. | Mengubah integer harga menjadi teks Rupiah. |

Kata `mod` mendaftarkan file sebagai modul. Kata `pub` pada `laptop_response` membuka jalur modul untuk handler. `rupiah` tetap privat; fungsi formatter-nya hanya diekspos ke modul induk dan turunannya dengan `pub(super)`.

## Hubungan antar-presenter

```text
Laptop dari repository
       ↓
laptop_response::laptop_to_array / laptop_to_object
       ↓ memakai
rupiah::formatkan_rupiah
       ↓
serde_json::Value untuk handler
```

GET daftar, GET detail, dan PATCH memakai array berurutan `[id, nama, merek, harga, stok]`. POST memakai object dengan nama field. Perbedaan tersebut adalah kontrak API saat ini; handler menentukan kapan setiap presenter dipakai.

## File terkait

- [Presenter laptop](laptop-response.md) menjelaskan pemetaan field dan bentuk JSON.
- [Formatter Rupiah](rupiah.md) menjelaskan tanda dan pemisah ribuan.
- [Handler create](../handlers/laptops/create.md) memakai object response.
- [Handler read](../handlers/laptops/read.md) dan [update](../handlers/laptops/update.md) memakai array response.
