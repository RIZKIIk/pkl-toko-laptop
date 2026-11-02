# Validasi input

> **Navigasi:** [Indeks catatan](../README.md) · [README proyek](../../README.md) · [Endpoint API](../../endpoin.md)

[Buka modul validation](../../src/validation/mod.rs)

## Tanggung jawab

Validasi memastikan nilai dari request layak diproses sebelum handler memanggil repository. Aturan dipisahkan dari handler supaya alur HTTP tidak bercampur dengan pemeriksaan nilai.

## Struktur

```text
validation/
├── mod.rs
├── id.rs
├── text.rs
├── create.rs
└── update.rs
```

| Catatan | Aturan |
| --- | --- |
| [validation/mod.rs](validation/mod.md) | Mendaftarkan aturan dan mengekspor validator publik. |
| [validation/id.rs](validation/id.md) | ID harus lebih besar dari nol. |
| [validation/text.rs](validation/text.md) | Menghapus spasi pinggir, menolak teks kosong, dan membatasi karakter. |
| [validation/create.rs](validation/create.md) | Memeriksa seluruh payload POST. |
| [validation/update.rs](validation/update.md) | Memeriksa field yang dikirim pada PATCH dan menolak payload kosong. |

## Perilaku umum

Validator mengembalikan `Result`. `Ok(...)` berisi nilai yang sudah valid dan/atau dinormalisasi. `Err(ApiError::BadRequest(...))` diteruskan handler dan dipetakan menjadi HTTP 400 oleh [error handler](error.md).

Validasi aplikasi memberi pesan yang mudah dipahami client. Constraint pada PostgreSQL di [migration](database/0001-create-laptops.md) tetap menjadi perlindungan terakhir untuk data yang ditulis melalui jalur lain.
