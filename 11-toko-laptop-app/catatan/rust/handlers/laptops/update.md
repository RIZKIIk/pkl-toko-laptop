# `handlers/laptops/update.rs`

> **Navigasi:** [Indeks catatan](../../../README.md) · [README proyek](../../../../README.md) · [Endpoint API](../../../../endpoin.md)

[Buka source `update.rs`](../../../../src/handlers/laptops/update.rs)

## Tujuan

`update_laptop` menangani `PATCH /api/laptops/{id}` untuk mengubah satu atau beberapa field.

## Extractor dan hasil

- `State(pool)` mengambil pool bersama dari router.
- `Path(id)` membaca ID dari URL.
- `Json(payload)` mendeserialisasi body menjadi `UpdateLaptopPayload`, yang field-nya opsional.
- Hasil sukses berupa `Json<Value>` dengan status, pesan, dan representasi laptop. Kesalahan diteruskan sebagai `ApiError`.

## Alur

1. Validasi ID agar query tidak menerima nol atau ID negatif.
2. Validasi payload agar teks dibersihkan, angka tidak negatif, dan setidaknya satu field dikirim.
3. Repository menjalankan UPDATE parsial. Field `None` tidak mengubah nilai lama.
4. Jika hasil repository `None`, `.ok_or_else(...)` membuat error 404.
5. Presenter membentuk array data terbaru; helper response menambahkan pesan `Laptop berhasil diperbarui`.

Pemeriksaan detail ada di [validation update](../../validation/update.md), query di [repository laptop](../../repositories/laptop-repository.md).
