# `handlers/laptops/mod.rs`

> **Navigasi:** [Indeks catatan](../../../README.md) · [README proyek](../../../../README.md) · [Endpoint API](../../../../endpoin.md)

[Buka source `laptops/mod.rs`](../../../../src/handlers/laptops/mod.rs)

## Tujuan

Mengelompokkan handler resource laptop, mengekspor fungsi untuk router, dan menyediakan helper response bersama.

## Modul dan ekspor

Deklarasi `mod create;`, `mod delete;`, `mod read;`, dan `mod update;` memberi nama pada file implementasi di folder yang sama. Tanpa `pub`, modul implementasi tidak terbuka langsung untuk pemanggil luar.

`pub use create::create_laptop;` (dan deklarasi sejenis) mengekspor ulang fungsi tertentu. Router dapat memakai jalur `handlers::laptops::create_laptop` tanpa mengetahui file internalnya. `pub use read::{get_laptop, list_laptops};` memakai kurung kurawal untuk mengekspor dua nama dari satu modul.

## Helper response

- `success_response(data)` membentuk JSON dengan `status` dan `data`.
- `success_response_with_message(message, data)` menambahkan field `message` untuk operasi create dan update.
- `not_found_laptop(id)` membuat `ApiError::NotFound` dengan format pesan yang seragam.

`Value` adalah tipe umum nilai JSON dan `json!` menyusunnya. Tiga helper memakai `pub(super)`: fungsi dapat diakses dari modul induk `handlers` dan submodulnya, tetapi tidak diekspos sebagai API publik crate. `message: &str` meminjam teks pesan; `data: Value` menerima body yang sudah dibentuk presenter. `not_found_laptop(id: i64) -> ApiError` menghasilkan error 404 dan memakai `format!` untuk menyisipkan ID ke dalam pesan.

## File operasi

Lihat [create](create.md), [read](read.md), [update](update.md), dan [delete](delete.md). Daftar endpoint dan bentuk body dijelaskan di [catatan handlers](../../handlers.md).
