# `src/validation/mod.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

[Buka source `validation/mod.rs`](../../../src/validation/mod.rs)

## Tujuan

File ini mengatur modul validator dan menentukan fungsi validasi mana yang boleh dipanggil langsung oleh handler. Ia tidak melakukan validasi sendiri; implementasinya ada di file validator terkait.

## 1. Deklarasi modul internal

```rust
mod create;
mod id;
mod text;
mod update;
```

Setiap `mod nama;` memberi tahu Rust bahwa ada modul di file `nama.rs` dalam folder `validation/`.

- `create` memeriksa payload untuk membuat laptop.
- `id` memastikan ID laptop positif.
- `text` membersihkan dan membatasi teks; ia dipakai oleh validator payload.
- `update` memeriksa payload perubahan parsial.

Tidak adanya `pub` berarti modul-modul ini tidak dibuka langsung ke pemakai crate melalui nama `validation::create`, `validation::text`, dan seterusnya. Handler diarahkan memakai fungsi yang diekspor di bawah.

## 2. Ekspor fungsi validator

```rust
pub use create::validate_create_laptop;
pub use id::validate_laptop_id;
pub use update::validate_update_laptop;
```

- `use` membawa nama dari modul sumber ke namespace `validation`.
- `pub use` juga mengekspor ulang nama itu agar dapat dipakai modul lain.
- Contohnya, `pub use id::validate_laptop_id;` memungkinkan handler memanggil `crate::validation::validate_laptop_id(...)`, tanpa menyebut jalur internal `crate::validation::id::...`.

Hanya fungsi yang diperlukan handler yang diekspor. Helper teks tetap detail internal karena tidak ada `pub use text::...`.

## Alur pemakaian

```text
handler
  -> crate::validation::validate_create_laptop(...)
     -> validator create memakai helper text secara internal
  -> lanjut ke repository hanya jika hasilnya Ok
```

Jika fungsi validasi menghasilkan `Err(ApiError)`, handler umumnya memakai operator `?` untuk menghentikan proses dan meneruskan error ke mekanisme response aplikasi.

## Catatan per validator

- [ID](id.md): aturan nilai path positif.
- [Teks](text.md): trim, penolakan teks kosong, dan batas karakter.
- [Create](create.md): validasi seluruh field laptop baru.
- [Update](update.md): validasi field yang dikirim untuk perubahan parsial.
