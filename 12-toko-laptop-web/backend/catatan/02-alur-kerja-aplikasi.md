# Alur Kerja Aplikasi

> **Navigasi:** [Indeks catatan](README.md) · [README proyek](../README.md) · [Endpoint API](../endpoin.md)

![Ilustrasi: Satu request, melewati 6 lapisan](ilustrasi/halaman/03-02-alur-kerja-aplikasi.svg "Satu request, melewati 6 lapisan")


Halaman ini menghubungkan file-file backend. Penjelasan isi setiap fungsi ada di catatan file masing-masing; halaman ini fokus pada urutan perpindahan data.

## Di halaman ini

[Startup](#saat-server-dinyalakan) · [Request umum](#perjalanan-umum-request) · [Login](#alur-login-session) · [Error](#jalur-error) · [Endpoint laptop](#alur-setiap-endpoint) · [Lanjut belajar](#lanjut-belajar)

## Saat server dinyalakan

1. [main.rs](rust/main.md) mencoba memuat `.env` dengan `dotenvy::dotenv().ok()`.
2. `AppConfig::baca_dari_env()` di [config.rs](rust/config.md) membaca host, port, kredensial PostgreSQL/admin, URL Redis, dan secret JWT.
3. Startup membuka `PgPool` dan koneksi Redis. Redis dipakai sebagai session store dan harus tersedia sebelum server mulai menerima request.
4. SQLx menjalankan migration yang belum tercatat di `_sqlx_migrations`. Migration audit membuat/menyesuaikan `laptop_logs` dan memasang trigger pada `laptops`; lihat [catatan skema database](rust/database/initial-schema.md). Buat backup sebelum startup menerapkan perubahan pada database lama.
5. `migrasikan_kata_sandi_lama()` meng-hash password lama dengan Argon2id dan menghapus nilai lama; `siapkan_pengguna_admin()` menyimpan hash password akun dari environment.
6. `routes::buat_router(...)` di [routes.rs](rust/routes.md) memasang endpoint, middleware session/JWT, `RedisStore`, dan pool PostgreSQL sebagai state router.
7. Tokio membuat `TcpListener` pada `APP_HOST:APP_PORT`.
8. `axum::serve(listener, app).await` menunggu koneksi HTTP.

```text
.env / environment -> AppConfig -> PgPool + Redis -> SQLx migrations -> users table + seed akun latihan -> Router + Redis session store + JWT secret -> TcpListener -> Axum
```

## Perjalanan umum request

```text
Postman / client
        ↓ method + URL + headers + body HTTP
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

`State(pool)` mengambil pool yang dipasang router; `Path(id)` membaca `{id}` dari URL; `Json(payload)` meminta Serde mengubah body JSON menjadi struct dengan field dan tipe yang sesuai. Extractor `Json`/`Path` menolak body atau parameter yang bentuknya tidak cocok sebelum handler dipanggil. Header, cookie, dan body adalah bagian request yang berbeda; login memberi client JWT serta session cookie.

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
  -> middleware wajibkan_autentikasi memverifikasi JWT/session dan role pengguna di database
  -> logout menghapus session aktif
  -> JWT tetap berlaku sampai waktu kedaluwarsanya
```

Password di database berupa hash Argon2id dengan salt unik. Server-side session berarti browser atau `curl` membawa cookie sebagai penunjuk session; username disimpan di Redis, bukan dipercayakan dari body request pada setiap endpoint.

JWT berlaku satu jam sejak login dan diterima sampai waktu `exp` selama signature valid dan akun masih ada. Middleware mengambil role terkini dari database untuk memeriksa izin. Logout hanya menghapus session yang dipakai pada request; JWT bearer tidak dicabut dan tetap berlaku sampai kedaluwarsa.

## Jalur error

| Keadaan | Asal | Bentuk hasil |
| --- | --- | --- |
| Input laptop tidak valid (ID nol/negatif, teks kosong, angka negatif, PATCH kosong) | `validation/` | `ApiError::BadRequest` -> 400 JSON error. Validasi username/password auth juga menghasilkan 400, melalui response auth. |
| ID valid tetapi laptop tidak ada | Repository memberi `None` atau `false`. | `ApiError::NotFound` -> 404 JSON error. |
| Koneksi/query database gagal | SQLx | `ApiError::Database` -> 500 JSON error; detail dicetak di server. |
| Kredensial login salah atau tidak ada session/JWT valid | `auth` handler/middleware | 401 JSON error. |
| Username registrasi sudah dipakai | Auth handler menerima `false` dari insert. | 409 `Conflict`. |
| Role tidak diizinkan untuk endpoint | Middleware role | 403 `Forbidden`. |
| Penyimpanan atau pembacaan session gagal | `auth` handler/middleware | 500 JSON error; detail dicatat di server. |
| JSON request/path tidak sesuai tipe | Extractor Axum | Axum menghentikan request sebelum handler. |

Status HTTP terpisah dari body JSON: `2xx` menunjukkan sukses, `4xx` masalah request/akses/resource, dan `5xx` kegagalan server. Misalnya `201 Created` dipakai saat insert berhasil, sementara body response membawa data yang dibuat.

## Alur setiap endpoint

### `GET /health`

```text
route -> handlers::system::health -> Json({ status: "ok", service: ... })
```

Endpoint ini tidak memakai pool dan tidak menjalankan query. Cocok untuk mengecek apakah server HTTP hidup.

### `GET /api/laptops`

```text
route
  -> handlers::laptops::daftar_laptop
  -> repositories::laptop_repository::daftar_laptop
  -> SELECT ... ORDER BY id
  -> Vec<Laptop>
  -> presenter laptop_ke_array untuk tiap laptop
  -> { status: "success", data: [ [...], ... ] }
```

Daftar kosong tetap response sukses; `data` menjadi array kosong.

### `GET /api/laptops/{id}`

```text
Path<i64>
  -> validasi_id_laptop
  -> repositories::laptop_repository::ambil_laptop
  -> Some(Laptop) / None
  -> ok_or_else(laptop_tidak_ditemukan)
  -> laptop_ke_array
  -> response sukses / 404
```

Data detail berbentuk array `[id, nama, merek, harga_rupiah, stok]`, bukan object. ID yang nol/negatif berhenti di validasi (400); ID positif tetapi tidak ada menjadi 404.

### `POST /api/laptops`

```text
Json<CreateLaptopPayload>
  -> validasi_laptop_baru
  -> INSERT ... RETURNING
  -> Laptop baru
  -> laptop_ke_objek
  -> 201 Created + message
```

Validasi merapikan nama/merek serta memeriksa harga/stok. `RETURNING` memberi ID yang dibuat database agar dapat dikirim dalam response. POST adalah endpoint yang memakai JSON object bernama untuk `data`.

### `PATCH /api/laptops/{id}`

```text
Path<i64> + Json<UpdateLaptopPayload>
  -> validasi ID dan field yang ada
  -> UPDATE memakai COALESCE
  -> Some(Laptop) / None
  -> ok_or_else(laptop_tidak_ditemukan)
  -> laptop_ke_array + message
```

`Option` di payload membedakan field yang dikirim dan tidak. `COALESCE` di SQL memakai nilai lama bila parameter dari `Option` adalah `None`. Body kosong ditolak agar PATCH tidak hanya mengubah `updated_at`.

### `DELETE /api/laptops/{id}`

```text
Path<i64>
  -> validasi_id_laptop
  -> DELETE ... WHERE id = $1
  -> rows_affected > 0
  -> response sukses / 404
```

Endpoint ini tidak memakai presenter karena response hanya membawa status, pesan, dan ID yang dihapus.

## Lanjut belajar

Untuk melihat pasangan method/URL yang tepat, buka [routes](rust/routes.md). Untuk melihat handler per operasi, buka [handlers](rust/handlers.md) lalu [handler laptop](rust/handlers/laptops/mod.md). Contoh request ada di [endpoin.md](../endpoin.md) dan [Postman collection](../postman/toko-laptop-app.postman_collection.json).
