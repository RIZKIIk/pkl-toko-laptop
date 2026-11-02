# Model dan payload

> **Navigasi:** [Indeks catatan](../README.md) · [README proyek](../../README.md) · [Endpoint API](../../endpoin.md)

[Buka modul model](../../src/models/mod.rs)

## Tanggung jawab

Model mendeskripsikan data yang berpindah di antara handler, validasi, repository, dan presenter. Model tidak menjalankan query dan tidak membuat response HTTP.

## Struktur

```text
models/
├── mod.rs
└── laptop/
    ├── mod.rs
    ├── entity.rs
    ├── create_payload.rs
    └── update_payload.rs
```

| Catatan | Isi |
| --- | --- |
| [models/mod.rs](models/mod.md) | Modul dan re-export supaya import tetap ringkas. |
| [laptop/mod.rs](models/laptop/mod.md) | Pengelompokan tipe laptop. |
| [laptop/entity.rs](models/laptop/entity.md) | Model hasil query PostgreSQL, `Laptop`. |
| [laptop/create-payload.rs](models/laptop/create-payload.md) | Field wajib untuk request POST. |
| [laptop/update-payload.rs](models/laptop/update-payload.md) | Field opsional untuk request PATCH. |

## Cara membaca sintaks model

- `pub mod laptop;` pada file modul mendaftarkan submodul `laptop`, yang isinya berada di folder `models/laptop/`.
- `pub use laptop::{...};` mengekspor ulang nama tipe dari submodul. Karena itu file lain dapat menulis `models::Laptop` daripada `models::laptop::Laptop`.
- `pub` membuat item dapat digunakan melalui jalur modul yang juga publik.
- `#[derive(...)]` meminta compiler atau library menghasilkan implementasi trait yang umum secara otomatis.
- `FromRow` membantu SQLx membentuk struct dari kolom hasil query.
- `Deserialize` membantu Serde membaca JSON request menjadi struct Rust.
- `Option<T>` berarti nilai bertipe `T` bisa ada (`Some(value)`) atau tidak dikirim (`None`).

## Tiga bentuk data yang berbeda

| Tipe | Arah data | Kegunaan |
| --- | --- | --- |
| `Laptop` | Database → aplikasi | Merepresentasikan satu baris laptop yang sudah ada. Biasanya diisi SQLx dari hasil query melalui `FromRow`. |
| `CreateLaptopPayload` | Request JSON → aplikasi | Menampung field yang wajib dikirim saat membuat laptop. Serde mengisinya melalui `Deserialize`. |
| `UpdateLaptopPayload` | Request JSON → aplikasi | Menampung field yang boleh diubah. Field berupa `Option` agar field yang tidak dikirim bisa dibedakan dari field yang dikirim. |

`Deserialize` hanya memastikan JSON dapat dibaca sebagai tipe Rust. Itu belum memastikan nilainya sesuai aturan aplikasi. Contohnya, teks masih bisa kosong atau angka masih bisa negatif; aturan tersebut diperiksa oleh [validation](validation.md).

## Alur data singkat

```text
JSON request -> Payload -> Validation -> Repository -> Laptop -> Presenter -> JSON response
```

Payload adalah bentuk data masuk, `Laptop` adalah bentuk data hasil database, dan presenter menentukan bentuk akhir yang dikirim ke client.
