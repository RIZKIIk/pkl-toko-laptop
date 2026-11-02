# `src/models/laptop/entity.rs`

> **Navigasi:** [Indeks catatan](../../../README.md) · [README proyek](../../../../README.md) · [Endpoint API](../../../../endpoin.md)

[Buka source `entity.rs`](../../../../src/models/laptop/entity.rs)

## Tujuan

`Laptop` mewakili data satu baris dari tabel PostgreSQL `laptops`. Repository memakai tipe ini untuk hasil query, lalu presenter membaca field-nya untuk membentuk JSON response.

## Import

```rust
use sqlx::FromRow;
```

`use` membawa trait `FromRow` dari SQLx ke dalam file. Trait ini memungkinkan SQLx membentuk struct dari satu baris hasil query.

## Struct `Laptop`

```rust
#[derive(Debug, FromRow)]
pub struct Laptop {
    pub id: i64,
    pub nama: String,
    pub merek: String,
    pub harga: i64,
    pub stok: i32,
}
```

- `#[derive(Debug, FromRow)]` adalah atribut yang meminta compiler/library menghasilkan implementasi trait secara otomatis:
  - `Debug` memungkinkan nilai ditampilkan untuk keperluan debugging.
  - `FromRow` memungkinkan SQLx memetakan baris query ke field struct.
- `pub struct Laptop` mendeklarasikan tipe data bernama `Laptop` yang dapat dipakai modul lain.
- Kurung kurawal berisi field. Setiap field memiliki nama, titik dua, lalu tipe data. Koma memisahkan field.
- `pub` pada setiap field mengizinkan repository dan presenter lain membaca atau mengisi nilainya.

| Field | Tipe Rust | Makna |
| --- | --- | --- |
| `id` | `i64` | ID unik dari database (kolom `BIGINT`). |
| `nama` | `String` | Nama atau model laptop. |
| `merek` | `String` | Merek laptop. |
| `harga` | `i64` | Harga dalam rupiah utuh, bukan teks yang sudah diformat. |
| `stok` | `i32` | Jumlah unit yang tersedia. |

Timestamp yang ada pada tabel belum dimasukkan ke struct ini karena tidak dibutuhkan oleh response API saat ini.

## Hubungan dengan tipe lain

`Laptop` adalah bentuk data hasil database, berbeda dengan [CreateLaptopPayload](create-payload.md) dan [UpdateLaptopPayload](update-payload.md) yang membaca request JSON. [Presenter laptop](../../presenters/laptop-response.md) mengubah `Laptop` ke bentuk response yang digunakan API.
