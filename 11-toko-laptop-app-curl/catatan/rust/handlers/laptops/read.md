# `handlers/laptops/read.rs`

> **Navigasi:** [Indeks catatan](../../../README.md) · [README proyek](../../../../README.md) · [Endpoint API](../../../../endpoin.md)

[Buka source `read.rs`](../../../../src/handlers/laptops/read.rs)

## Tujuan

File ini menangani pembacaan daftar dan detail laptop.

## `list_laptops`

1. Mengambil `PgPool` melalui `State`.
2. Memanggil repository yang mengembalikan `Vec<Laptop>`.
3. `iter()` meminjam item satu per satu; `map(laptop_to_array)` mengubah tiap model; `collect::<Vec<_>>()` mengumpulkan hasil.
4. `json!(data)` membentuk array JSON dan `success_response` menambah status sukses.

Jika belum ada data, hasilnya array kosong, bukan error.

## `get_laptop`

1. Extractor `Path(id)` membaca ID URL sebagai `i64`.
2. `validate_laptop_id(id)?` menolak nol atau angka negatif sebelum query.
3. Repository mengembalikan `Result<Option<Laptop>, sqlx::Error>`. `await?` meneruskan error query, sedangkan `.ok_or_else(...)` mengubah `None` menjadi 404 dan mengambil nilai dari `Some`.
4. `laptop_to_array` menjaga format detail tetap `[id, nama, merek, harga, stok]`.

## Format dan route

Kedua endpoint mengembalikan status sukses dengan `data` array. Router memasangnya pada `GET /api/laptops` dan `GET /api/laptops/{id}`.
