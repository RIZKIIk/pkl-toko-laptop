# `src/models/mod.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

[Buka source `models/mod.rs`](../../../src/models/mod.rs)

## Tujuan

File ini menjadi pintu masuk modul model. Ia mendaftarkan tipe laptop dan mengekspor ulang tipe yang sering dipakai agar import di file lain lebih ringkas.

## Kode dan urutannya

```rust
pub mod laptop;

pub use laptop::{CreateLaptopPayload, Laptop, UpdateLaptopPayload};
```

### `pub mod laptop;`

- `mod` memberi tahu Rust bahwa ada submodul bernama `laptop`, yang source-nya didefinisikan pada folder `models/laptop/`.
- `pub` membuat submodul tersebut bisa dijangkau dari modul lain melalui `models::laptop`.
- Tanda titik koma menyatakan deklarasi modul berada di file terpisah, bukan berisi blok kode di sini.

### `pub use laptop::{...};`

- `use` membawa nama dari suatu jalur ke cakupan modul saat ini.
- `laptop::{...}` memakai sintaks kurung kurawal untuk memilih beberapa nama sekaligus dari modul `laptop`.
- `pub use` juga mengekspor ulang nama-nama tersebut agar modul lain dapat mengaksesnya melalui `crate::models::...`.

Contohnya:

```rust
use crate::models::Laptop;
```

Tanpa re-export, pemanggil harus memakai jalur lebih panjang:

```rust
use crate::models::laptop::Laptop;
```

Re-export tidak membuat salinan tipe dan tidak mengubah perilakunya; hanya menyediakan jalur import yang lebih pendek.

## Tipe yang diekspor

| Tipe | Kegunaan |
| --- | --- |
| `Laptop` | Satu laptop yang dibaca atau disimpan ke database. |
| `CreateLaptopPayload` | Data yang dikirim untuk membuat laptop baru. |
| `UpdateLaptopPayload` | Data perubahan parsial untuk laptop. |

Penjelasan field dan derive tiap tipe ada di modul [laptop](laptop/mod.md).
