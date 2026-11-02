# Alur Kerja Aplikasi

> **Navigasi:** [Indeks catatan](README.md) · [README proyek](../README.md) · [Endpoint API](../endpoin.md)

![Ilustrasi: Satu request, melewati 6 lapisan](ilustrasi/halaman/03-02-alur-kerja-aplikasi.svg "Satu request, melewati 6 lapisan")


Halaman ini menghubungkan file-file backend. Penjelasan isi setiap fungsi ada di catatan file masing-masing; halaman ini fokus pada urutan perpindahan data.

## Di halaman ini

[Startup](#saat-server-dinyalakan) · [Request umum](#perjalanan-umum-request) · [Login](#alur-login-session) · [Error](#jalur-error) · [Endpoint laptop](#alur-setiap-endpoint) · [Lanjut belajar](#lanjut-belajar)

## Saat server dinyalakan

1. [main.rs](rust/main.md) mencoba memuat `.env` dengan `dotenvy::dotenv().ok()`.
2. `AppConfig::from_env()` di [config.rs](rust/config.md) membaca host, port, kredensial PostgreSQL/admin, dan secret JWT.
3. `connect_database().await` membuat `PgPool`. Bila langkah ini gagal, server belum mulai menerima request.
4. SQLx menjalankan seluruh migration dan mencatat migration yang sudah diaplikasikan di `_sqlx_migrations`.
5. `initialize_admin_user()` menyimpan akun latihan dari environment. Password disimpan langsung untuk memudahkan materi session; jangan gunakan cara ini untuk produksi.
6. `routes::create_router(...)` di [routes.rs](rust/routes.md) memasang endpoint, middleware session/JWT, dan pool sebagai state router.
7. Tokio membuat `TcpListener` pada `APP_HOST:APP_PORT`.
8. `axum::serve(listener, app).await` menunggu koneksi HTTP.

```text
.env / environment -> AppConfig -> PgPool -> SQLx migrations -> users table + seed akun latihan -> Router + session store + JWT secret -> TcpListener -> Axum
```

## Perjalanan umum request

```text
Postman / client
        ↓ HTTP method + URL + body
routes.rs memilih handler
        ↓
middleware menerima session cookie atau JWT bearer untuk endpoint laptop dan endpoint session/logout
        ↓
handlers/{auth,system,laptops} mengambil Session / State / Path / Json
        ├─ validation/{id,create,update,text} (bila ada input)
        ├─ repositories/{laptop,user}_repository -> PostgreSQL -> model/hasil query
        └─ presenters/{laptop_response,rupiah} (bila response membawa laptop)
        ↓
Json response atau ApiError
        ↓
client menerima status HTTP + JSON
```

`State(pool)` mengambil pool dari router. `Path(id)` membaca `{id}` dari URL. `Json(payload)` mencoba membaca body JSON menjadi struct. Bila JSON atau tipe path tidak dapat dibaca Axum, handler tidak dipanggil karena request gagal pada tahap extractor.

## Alur login session

```text
POST /api/auth/register + username/password
  -> validasi username dan password
  -> auth service membuat akun baru di users
  -> username duplikat: 409; berhasil: 201 Created
  -> pengguna login terpisah untuk membuat session
```

```text
POST /api/auth/login + username/password
  -> handler login memeriksa input
  -> auth service mencari password username di users
  -> password cocok: buat session baru dan JWT HS256 (berlaku satu jam)
  -> SessionManagerLayer mengirim cookie session ke client
  -> client mengirim cookie session atau Authorization: Bearer <JWT>
  -> middleware require_auth memverifikasi JWT atau membaca session
  -> logout menghapus session; JWT tetap berlaku sampai kedaluwarsa
```

Password di database berupa teks biasa hanya untuk latihan lokal session. Server-side session berarti browser atau `curl` membawa cookie sebagai penunjuk session; username disimpan di session store, bukan dipercayakan dari body request pada setiap endpoint.

## Jalur error

| Keadaan | Asal | Bentuk hasil |
| --- | --- | --- |
| ID nol/negatif, teks kosong, angka negatif, PATCH kosong | `validation/` | `ApiError::BadRequest` -> 400 JSON error. |
| ID valid tetapi laptop tidak ada | Repository memberi `None` atau `false`. | `ApiError::NotFound` -> 404 JSON error. |
| Koneksi/query database gagal | SQLx | `ApiError::Database` -> 500 JSON error; detail dicetak di server. |
| Kredensial login salah atau tidak ada session/JWT valid | `auth` handler/middleware | 401 JSON error. |
| Penyimpanan atau pembacaan session gagal | `auth` handler/middleware | 500 JSON error; detail dicatat di server. |
| JSON request/path tidak sesuai tipe | Extractor Axum | Axum menghentikan request sebelum handler. |

## Alur setiap endpoint

### `GET /health`

```text
route -> handlers::system::health -> Json({ status: "ok", service: ... })
```

Endpoint ini tidak memakai pool dan tidak menjalankan query. Cocok untuk mengecek apakah server HTTP hidup.

### `GET /api/laptops`

```text
route
  -> handlers::laptops::list_laptops
  -> repositories::laptop_repository::list_laptops
  -> SELECT ... ORDER BY id
  -> Vec<Laptop>
  -> presenter laptop_to_array untuk tiap laptop
  -> { status: "success", data: [ [...], ... ] }
```

Daftar kosong tetap response sukses; `data` menjadi array kosong.

### `GET /api/laptops/{id}`

```text
Path<i64>
  -> validate_laptop_id
  -> repositories::laptop_repository::get_laptop
  -> Some(Laptop) / None
  -> ok_or_else(not_found_laptop)
  -> laptop_to_array
  -> response sukses / 404
```

Data detail berbentuk array `[id, nama, merek, harga_rupiah, stok]`, bukan object. ID yang nol/negatif berhenti di validasi (400); ID positif tetapi tidak ada menjadi 404.

### `POST /api/laptops`

```text
Json<CreateLaptopPayload>
  -> validate_create_laptop
  -> INSERT ... RETURNING
  -> Laptop baru
  -> laptop_to_object
  -> 201 Created + message
```

Validasi merapikan nama/merek serta memeriksa harga/stok. `RETURNING` memberi ID yang dibuat database agar dapat dikirim dalam response. POST adalah endpoint yang memakai JSON object bernama untuk `data`.

### `PATCH /api/laptops/{id}`

```text
Path<i64> + Json<UpdateLaptopPayload>
  -> validasi ID dan field yang ada
  -> UPDATE memakai COALESCE
  -> Some(Laptop) / None
  -> ok_or_else(not_found_laptop)
  -> laptop_to_array + message
```

`Option` di payload membedakan field yang dikirim dan tidak. `COALESCE` di SQL memakai nilai lama bila parameter dari `Option` adalah `None`. Body kosong ditolak agar PATCH tidak hanya mengubah `updated_at`.

### `DELETE /api/laptops/{id}`

```text
Path<i64>
  -> validate_laptop_id
  -> DELETE ... WHERE id = $1
  -> rows_affected > 0
  -> response sukses / 404
```

Endpoint ini tidak memakai presenter karena response hanya membawa status, pesan, dan ID yang dihapus.

## Lanjut belajar

Untuk melihat pasangan method/URL yang tepat, buka [routes](rust/routes.md). Untuk melihat handler per operasi, buka [handlers](rust/handlers.md) lalu [handler laptop](rust/handlers/laptops/mod.md). Contoh request ada di [endpoin.md](../endpoin.md) dan [Postman collection](../postman/toko-laptop-app.postman_collection.json).
