# Handler HTTP

> **Navigasi:** [Indeks catatan](../README.md) · [README proyek](../../README.md) · [Endpoint API](../../endpoin.md)

![Ilustrasi: Handler sebagai pengarah request](../ilustrasi/halaman/25-handlers.svg "Handler sebagai pengarah request")


[Buka modul handler](../../src/handlers/mod.rs)

## Peran handler

Handler menjadi penghubung antara request HTTP dan logika aplikasi. Handler membaca nilai dari URL, body JSON, atau state Axum; memvalidasi input; memanggil repository; lalu membentuk response.

Handler tidak menyusun query SQL. Pembagian ini membuat alur request mudah diikuti dan menghindari detail database bercampur dengan detail HTTP.

## Struktur modul

```text
handlers/
├── mod.rs                 # Mengekspor modul resource
├── auth.rs                # Registrasi, login, identitas, dan logout
├── system.rs              # Health check HTTP
└── laptops/
    ├── mod.rs             # Ekspor handler laptop dan helper response
    ├── create.rs          # POST
    ├── read.rs            # GET daftar dan detail
    ├── update.rs          # PATCH
    └── delete.rs          # DELETE
```

| Catatan dan source | Tanggung jawab |
| --- | --- |
| [auth.rs](handlers/auth.md) | Registrasi, login, baca identitas session/JWT, dan logout. |
| [system.rs](handlers/system.md) | `GET /health`; hanya memeriksa server HTTP, bukan database. |
| [laptops/mod.rs](handlers/laptops/mod.md) | Modul, helper response sukses, dan pesan 404. |
| [laptops/create.rs](handlers/laptops/create.md) | Validasi dan pembuatan laptop. |
| [laptops/read.rs](handlers/laptops/read.md) | Membaca semua laptop atau satu laptop. |
| [laptops/update.rs](handlers/laptops/update.md) | Validasi dan penerapan perubahan parsial. |
| [laptops/delete.rs](handlers/laptops/delete.md) | Penghapusan berdasarkan ID. |

## Alur request umum

1. Axum memilih handler berdasarkan route dan method di [routes.rs](routes.md).
2. Extractor `State`, `Path`, dan `Json` mengambil pool, ID, atau payload.
3. Handler meminta [validation](validation.md) memeriksa input yang relevan.
4. Handler memanggil [repository](repositories.md) untuk operasi PostgreSQL.
5. [Presenter](presenters.md) mengubah model hasil query ke format response API.
6. Error aplikasi diteruskan sebagai [`ApiError`](error.md), lalu Axum mengubahnya ke status dan JSON.

## Sintaks yang sering muncul di handler

| Sintaks | Arti |
| --- | --- |
| `async fn` | Fungsi dapat menunggu operasi I/O seperti query database atau operasi session. |
| `State(pool): State<PgPool>` | Extractor Axum mengambil state bersama dari router, lalu pola destructuring memberi nama `pool` pada nilai yang diambil. |
| `Path(id): Path<i64>` | Membaca parameter path, misalnya `{id}`, lalu meminta Axum mengubahnya menjadi `i64`. |
| `Json(payload): Json<T>` | Membaca body JSON dan mengubahnya menjadi tipe `T` melalui Serde. |
| `&pool` / `&payload` | Meminjam nilai agar fungsi yang dipanggil bisa menggunakannya tanpa mengambil kepemilikan. |
| `.await` | Menunggu hasil pemanggilan asinkron sebelum melanjutkan. |
| `?` | Jika hasil `Result` adalah `Err`, hentikan fungsi dan teruskan error; jika `Ok`, lanjut dengan nilainya. |
| `Ok(value)` / `Err(error)` | Mewakili hasil sukses atau gagal pada `Result`. |
| `Option<T>` | Mewakili nilai yang mungkin ada (`Some`) atau tidak ada (`None`), misalnya baris yang tidak ditemukan. |
| `map(...)` | Mengubah setiap item di iterator dengan fungsi yang diberikan. |
| `collect::<Vec<_>>()` | Mengumpulkan hasil iterator menjadi `Vec`; `_` meminta compiler menyimpulkan tipe item. |
| `return` | Mengakhiri fungsi segera dengan nilai yang disebut sesudahnya. |

## Kontrak response yang dipertahankan

| Endpoint | Sukses | Bentuk body |
| --- | --- | --- |
| `GET /health` | 200 | Object status layanan. |
| `POST /api/auth/register` | 201 | Username akun yang dibuat. |
| `POST /api/auth/login` | 200 | Username, JWT, tipe token, dan masa berlaku; session cookie dikirim melalui header. |
| `GET /api/auth/session` | 200 | Username dari JWT atau session aktif. |
| `POST /api/auth/logout` | 200 | Pesan sukses; JWT yang ada tidak dicabut. |
| `GET /api/laptops` | 200 | `data` berupa daftar array laptop. |
| `GET /api/laptops/{id}` | 200 | `data` berupa satu array laptop. |
| `POST /api/laptops` | 201 | `data` berupa object laptop serta pesan. |
| `PATCH /api/laptops/{id}` | 200 | `data` berupa array laptop terbaru serta pesan. |
| `DELETE /api/laptops/{id}` | 200 | Status, pesan, dan ID yang dihapus. |

Data laptop pada array mengikuti urutan `[id, nama, merek, harga, stok]`.
