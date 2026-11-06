# `src/handlers/laptops/read.rs`

> **Navigasi:** [Indeks catatan](../../../README.md) · [README proyek](../../../../README.md) · [Endpoint API](../../../../endpoin.md)

![Ilustrasi: Dua cara membaca resource](../../../ilustrasi/halaman/21-read.svg "Dua cara membaca resource")


[Buka source `read.rs`](../../../../src/handlers/laptops/read.rs)

## Tujuan

File ini menyediakan dua operasi baca: `GET /api/laptops` untuk daftar dan `GET /api/laptops/{id}` untuk detail. Kedua handler mengambil data melalui repository, kemudian memakai presenter untuk menyiapkan bentuk JSON API.

## `list_laptops`

Signature-nya `async fn list_laptops(State(pool): State<PgPool>) -> Result<Json<Value>, ApiError>`. Handler tidak mempunyai body request atau path ID; satu-satunya input adalah pool yang diekstrak dari state router. `Result` berarti sukses menghasilkan `Json<Value>`, sedangkan `ApiError` diteruskan ke penanganan error aplikasi.

Alur handler daftar:

1. `State(pool)` mengambil pool dari state router.
2. `repository::list_laptops(&pool).await?` meminta semua baris, diurutkan berdasarkan ID.
3. Repository mengembalikan `Vec<Laptop>`. `iter()` meminjam setiap item; `map(laptop_to_array)` mengubahnya menjadi array response; `collect::<Vec<_>>()` mengumpulkan semua array.
4. `json!(data)` membentuk nilai JSON, lalu `success_response` membungkusnya dengan `status: "success"`.

Daftar kosong adalah kondisi sukses: response `200 OK` berisi `"data": []`.

Di sini `.await` menunggu operasi database yang asynchronous, lalu `?` memeriksa `Result`: jika query gagal, error dikembalikan dari handler; jika berhasil, nilai `Vec<Laptop>` menjadi `laptops`. `Vec<T>` adalah vector berukuran dinamis berisi nilai `T`. Pada `collect::<Vec<_>>()`, `Vec<_>` menyuruh compiler menyimpulkan tipe elemen dari hasil `map`.

## `get_laptop`

Signature-nya menerima `State(pool)` dan `Path(id): Path<i64>` serta mengembalikan `Result<Json<Value>, ApiError>`. `Path` meminta Axum membaca parameter URL `{id}` sebagai bilangan `i64`; nilai yang tidak dapat diubah menjadi integer ditolak oleh extractor sebelum fungsi berjalan.

Alur handler detail:

1. `Path(id)` membaca parameter `{id}` dari URL sebagai `i64`.
2. `validate_laptop_id(id)?` menolak nilai nol atau negatif sebelum query.
3. `repository::get_laptop(...)` mengembalikan `Result<Option<Laptop>, sqlx::Error>`. `Result` membedakan query berhasil/gagal; `Option` membedakan baris ditemukan (`Some(laptop)`) atau tidak ditemukan (`None`).
4. `.await?` berarti menunggu query dahulu, lalu meneruskan `sqlx::Error` jika gagal. `.ok_or_else(...)` mengubah `None` menjadi error 404 dan menghasilkan `Laptop` dari `Some(laptop)`. Closure `|| not_found_laptop(id)` hanya dipanggil bila nilainya `None`.
5. `laptop_to_array` membentuk data response dan `success_response` menambahkan status sukses.

Rangkaian `await?` kemudian `ok_or_else` membuat setiap kasus terlihat jelas tanpa `match` eksplisit: kegagalan query menjadi error database, baris yang hilang menjadi not-found, dan baris yang ada berlanjut ke response.

ID positif yang tidak ada menghasilkan `404 Not Found`; ID nol/negatif menghasilkan `400 Bad Request`.

## Bentuk response

Kedua handler mengirim laptop sebagai array berurutan `[id, nama, merek, harga, stok]`. Harga telah diformat menjadi string Rupiah.

```json
{
  "status": "success",
  "data": [1, "ThinkPad E14", "Lenovo", "Rp 12.500.000", 5]
}
```

Daftar memakai array of arrays pada `data`; detail memakai satu array. Lihat [kontrak presenter](../../presenters/laptop-response.md) untuk alasan dan pemetaan field.

## Hubungan dengan file lain

- [Routes](../../routes.md) memasang kedua fungsi pada method `GET`.
- [Validator ID](../../validation/id.md) memeriksa parameter detail.
- [Repository laptop](../../repositories/laptop-repository.md) menjalankan query SELECT.
- [Presenter laptop](../../presenters/laptop-response.md) menetapkan format array.
- [Helper handler laptop](mod.md) membungkus response sukses.

## Ringkasan endpoint

| Route | Input khusus | Hasil normal | Jika data tidak ada |
| --- | --- | --- | --- |
| `GET /api/laptops` | Tidak ada body atau path ID. | `200 OK`, `data` berisi array laptop. | Daftar kosong tetap `200 OK`. |
| `GET /api/laptops/{id}` | Path ID positif. | `200 OK`, `data` berisi satu array laptop. | `404 Not Found`. |

## Ilustrasi baca data

```text
GET /api/laptops       → SELECT semua → Vec<Laptop> → array of arrays
GET /api/laptops/{id}  → SELECT satu  → Option<Laptop> → array / 404
```
