# `handlers/mod.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

[Buka source `handlers/mod.rs`](../../../src/handlers/mod.rs)

## Tujuan

File ini mendaftarkan modul-modul handler. Ia tidak menjalankan request sendiri.

## Penjelasan kode

- `//!` mendeskripsikan modul dan memberi tautan ke catatan ini.
- `pub mod laptops;` membuat handler laptop tersedia melalui `handlers::laptops`.
- `pub mod system;` membuat endpoint pemeriksaan tersedia melalui `handlers::system`.
- `pub use ...;` juga mengekspor fungsi endpoint langsung pada `handlers::...`, sehingga [routes](../routes.md) dapat menggunakan satu namespace secara konsisten.

Rust mencari `laptops` dan `system` di subfolder `handlers/`, masing-masing melalui `mod.rs` dan `system.rs`. Detail tiap handler ada di halaman [system](system.md) dan [operasi laptop](laptops/mod.md).

## Posisi dalam alur

[routes.rs](../routes.md) menghubungkan URL ke fungsi yang diekspor dari modul-modul ini.
