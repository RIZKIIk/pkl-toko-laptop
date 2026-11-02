# `handlers/laptops/create.rs`

> **Navigasi:** [Indeks catatan](../../../README.md) · [README proyek](../../../../README.md) · [Endpoint API](../../../../endpoin.md)

[Buka source `create.rs`](../../../../src/handlers/laptops/create.rs)

## Tujuan dan signature

`create_laptop` menangani `POST /api/laptops`. Extractor `State(pool)` mengambil pool dari router, sedangkan `Json(payload)` meminta Axum membaca body sebagai `CreateLaptopPayload`.

Fungsi mengembalikan `Result<(StatusCode, Json<Value>), ApiError>`: bila berhasil ada status dan body JSON; bila gagal, `ApiError` dipetakan Axum menjadi response error.

## Alur

1. `validate_create_laptop(payload)?` membersihkan teks dan menolak nilai yang tidak valid. `?` menghentikan handler jika validasi gagal.
2. `repository::create_laptop(&pool, &payload).await?` menyimpan data dan mengambil baris yang dibuat. `await` menunggu operasi database.
3. `laptop_to_object(&laptop)` membentuk object JSON untuk response POST.
4. `success_response_with_message(...)` menambahkan status dan pesan berhasil.
5. `StatusCode::CREATED` memastikan POST yang berhasil memberi HTTP 201.

## Hubungan

Handler mengorkestrasikan [validasi create](../../validation/create.md), [repository](../../repositories/laptop-repository.md), [presenter](../../presenters/laptop-response.md), dan helper di [modul handler laptop](mod.md).
