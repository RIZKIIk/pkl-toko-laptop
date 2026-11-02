# `src/models/laptop/create_payload.rs`

> **Navigasi:** [Indeks catatan](../../../README.md) · [README proyek](../../../../README.md) · [Endpoint API](../../../../endpoin.md)

[Buka source `create_payload.rs`](../../../../src/models/laptop/create_payload.rs)

## Tujuan

`CreateLaptopPayload` adalah bentuk data yang digunakan Axum/Serde untuk membaca body JSON pada `POST /api/laptops`.

## Import

```rust
use serde::Deserialize;
```

`Deserialize` adalah trait Serde yang memungkinkan data masuk—dalam kasus ini JSON—diubah menjadi nilai Rust.

## Struct payload

```rust
#[derive(Debug, Deserialize)]
pub struct CreateLaptopPayload {
    pub nama: String,
    pub merek: String,
    pub harga: i64,
    pub stok: i32,
}
```

- `#[derive(Debug, Deserialize)]` meminta implementasi otomatis:
  - `Debug` untuk menampilkan nilai saat debugging.
  - `Deserialize` agar Serde dapat mengisi struct dari JSON.
- `pub struct` mendeklarasikan tipe data yang bisa dipakai bagian aplikasi lain.
- Field ditulis `nama: Tipe`; koma memisahkan satu field dengan yang berikutnya.
- `pub` pada field memungkinkan validator dan handler membaca nilainya.

Keempat field tidak memakai `Option`, sehingga body harus menyediakan semua nama field dengan tipe yang cocok:

| Field | Tipe | Contoh |
| --- | --- | --- |
| `nama` | `String` | `"ThinkPad E14"` |
| `merek` | `String` | `"Lenovo"` |
| `harga` | `i64` | `12500000` |
| `stok` | `i32` | `5` |

## Contoh JSON

```json
{
  "nama": "ThinkPad E14",
  "merek": "Lenovo",
  "harga": 12500000,
  "stok": 5
}
```

`Deserialize` hanya membaca JSON ke struct. Ia tidak mengecek apakah teks kosong atau angka negatif. Handler meneruskan payload ke [validator create](../../validation/create.md) sebelum repository menyimpan data.
