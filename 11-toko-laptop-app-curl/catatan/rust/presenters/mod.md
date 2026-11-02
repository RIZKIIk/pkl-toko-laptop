# `src/presenters/mod.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

[Buka source `presenters/mod.rs`](../../../src/presenters/mod.rs)

## Tujuan

File ini menyusun modul presenter. Presenter mengubah data model menjadi bentuk yang sesuai untuk response API, tetapi tidak mengirim response HTTP secara langsung.

## Kode dan urutannya

```rust
pub mod laptop_response;

mod rupiah;
```

### `pub mod laptop_response;`

Mendaftarkan submodul `laptop_response` yang implementasinya ada di `laptop_response.rs`. Kata `pub` membuat modul dan item publik di dalamnya dapat dijangkau melalui jalur seperti `presenters::laptop_response`.

### `mod rupiah;`

Mendaftarkan modul formatter Rupiah di `rupiah.rs`, tanpa kata `pub`. Modul ini dapat digunakan di dalam presenter yang menaunginya, tetapi tidak diekspos sebagai API publik crate.

Pada kedua baris, `mod` adalah deklarasi modul dan titik koma menunjukkan isi modul ditulis di file terpisah.

## Kenapa dipisah?

`laptop_response` menentukan apakah laptop menjadi object atau array JSON. `rupiah` hanya mengubah angka menjadi teks Rupiah. Detail implementasi ada di [laptop response](laptop-response.md) dan [formatter Rupiah](rupiah.md).
