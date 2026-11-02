# `src/validation/create.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

[Buka source `create.rs`](../../../src/validation/create.rs)

## Tujuan

Validator ini memeriksa payload sebelum handler meminta repository menyimpan laptop baru. Ia membersihkan nama dan merek, menolak teks kosong atau terlalu panjang, dan menolak harga maupun stok negatif.

## 1. Import

```rust
use crate::{error::ApiError, models::CreateLaptopPayload};
use super::text::validate_text;
```

- `crate::` memulai jalur dari akar crate.
- `crate::{...}` mengimpor beberapa nama dari akar crate dalam satu blok.
- `error::ApiError` adalah tipe error aplikasi untuk menandai input tidak valid.
- `models::CreateLaptopPayload` adalah struct yang mewakili body request POST.
- `super::text::validate_text` mengambil helper dari modul saudara `text` melalui modul induk `validation`.

## 2. Fungsi `validate_create_laptop`

```rust
pub fn validate_create_laptop(
    payload: CreateLaptopPayload,
) -> Result<CreateLaptopPayload, ApiError> {
    let nama = validate_text(payload.nama, "Nama laptop", 150)?;
    let merek = validate_text(payload.merek, "Merek laptop", 100)?;

    if payload.harga < 0 {
        return Err(ApiError::BadRequest("Harga tidak boleh negatif".into()));
    }
    if payload.stok < 0 {
        return Err(ApiError::BadRequest("Stok tidak boleh negatif".into()));
    }

    Ok(CreateLaptopPayload {
        nama,
        merek,
        harga: payload.harga,
        stok: payload.stok,
    })
}
```

### Deklarasi

- `pub` mengizinkan handler lain memanggil fungsi ini.
- `fn` mendeklarasikan fungsi.
- `payload: CreateLaptopPayload` menerima seluruh data request sebagai satu struct.
- `-> Result<CreateLaptopPayload, ApiError>` berarti mengembalikan payload yang sudah valid atau error aplikasi.
- `{ ... }` membatasi badan fungsi.

### Pemeriksaan berurutan

1. `payload.nama` dipindahkan ke helper `validate_text` bersama label `"Nama laptop"` dan batas 150 karakter. Hasil teks yang sudah dibersihkan disimpan dalam `nama`.
2. `?` setelah pemanggilan helper berarti jika validasi teks gagal, fungsi langsung mengembalikan `Err` kepada handler; jika berhasil, proses berlanjut.
3. Merek diperiksa dengan cara sama, dengan batas 100 karakter.
4. `if payload.harga < 0` menolak harga negatif. Harga nol diperbolehkan.
5. `return Err(...)` langsung mengakhiri fungsi. `ApiError::BadRequest(...)` menyatakan masalah request; `.into()` mengubah string literal menjadi `String` yang diperlukan pesan error.
6. Pemeriksaan stok menerapkan aturan yang sama: angka negatif ditolak, nol diperbolehkan.
7. `Ok(CreateLaptopPayload { ... })` membuat nilai payload baru dari data valid. `nama` dan `merek` memakai *field init shorthand*: jika nama field sama dengan nama variabel, cukup tulis namanya sekali. Harga dan stok disalin dari payload semula.

Jika seluruh pemeriksaan lolos, handler menerima payload yang teksnya sudah di-trim dan dapat meneruskannya ke repository.

## Contoh

Input nama `" ThinkPad "` akan menjadi `"ThinkPad"`. Merek `" Lenovo "` menjadi `"Lenovo"`. Harga `-1`, stok `-1`, teks kosong, atau teks yang melewati batas akan menghasilkan `BadRequest`.

## 3. Unit test

Modul test pada source memiliki dua kasus:

- Payload valid memastikan spasi pinggir nama dan merek dihapus.
- Payload dengan harga negatif memastikan nilai tersebut ditolak.

`#[cfg(test)]` membatasi modul test agar dikompilasi untuk test. `#[test]` menandai fungsi test. `assert_eq!` membandingkan hasil aktual dengan nilai harapan, sedangkan `.is_err()` memeriksa bahwa hasil bertipe `Result` adalah kegagalan.

Validator ini dipanggil oleh handler create sebelum query `INSERT`. Aturan umum teks dijelaskan pada [validate_text](text.md).
