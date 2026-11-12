# `src/auth/middleware.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

![Ilustrasi pemeriksaan middleware autentikasi](../../ilustrasi/halaman/51-auth-middleware.svg "JWT atau session diperiksa bersama identitas dan role")

[Buka source `middleware.rs`](../../../src/auth/middleware.rs)

## Tujuan dan cakupan

`require_auth` adalah middleware autentikasi sekaligus otorisasi untuk endpoint protected. Tugasnya memastikan request memiliki identitas pengguna yang masih berlaku, memastikan role pengguna boleh menjalankan method pada path tersebut, lalu meneruskan request ke handler.

Middleware tidak dipasang ke seluruh aplikasi. [`create_router`](../routes.md) menerapkannya dengan `route_layer` pada grup route protected: endpoint session dan logout, endpoint laptop, serta endpoint admin untuk pengguna. Endpoint health, login, dan register berada di luar grup tersebut.

Pemeriksaan dibagi ke beberapa bagian:

| Bagian | Tanggung jawab |
| --- | --- |
| [JWT](jwt.md) | Membaca bearer token serta memeriksa signature dan expiry. |
| [Session](session.md) | Membaca username dari session cookie. |
| `require_auth` | Memilih identitas, memeriksa ulang ke database, mengotorisasi role, dan meneruskan request. |
| [`UserRole::can_access`](../../../src/auth/role.rs) | Menentukan akses berdasarkan role, HTTP method, dan path. |

## Data yang diterima middleware

Axum menyediakan dependency berikut saat memanggil `require_auth`:

| Extractor | Sumber | Kegunaan |
| --- | --- | --- |
| `State<PgPool>` | State router | Memeriksa pengguna dan role ke database. |
| `Extension<JwtService>` | Extension router | Memverifikasi bearer JWT. |
| `Session` | `SessionManagerLayer` | Membaca identitas session dari cookie. |
| `Request` | Request masuk | Membaca header, method, dan path; juga menjadi tempat menyimpan identitas hasil pemeriksaan. |
| `Next` | Rantai middleware Axum | Menjalankan handler berikutnya setelah semua pemeriksaan berhasil. |

Identitas yang diteruskan ke handler berbentuk `AuthIdentity`, berisi `user_id`, `username`, dan `role`. Nilai ini bukan diambil begitu saja dari body request: middleware menyusunnya setelah verifikasi token atau pencarian username session di database.

## Urutan pemeriksaan

1. **Coba autentikasi JWT.** Middleware meminta claims dari header `Authorization` melalui `jwt::claims_from_headers`. Header harus memakai skema `Bearer` dan diikuti token tanpa spasi. Detail signature, expiry, dan claims ada di [catatan JWT](jwt.md).
2. **Cocokkan claims JWT dengan database.** Jika token lolos verifikasi, middleware mencari pengguna berdasarkan `sub`. Identitas diterima hanya jika pengguna masih ada, role dari database sama dengan role pada claims, dan `token_version` database sama dengan versi di token.
3. **Gunakan session bila JWT tidak menghasilkan identitas.** Token yang hilang, tidak valid/kedaluwarsa, tidak lagi cocok dengan data pengguna, atau pengguna tidak ditemukan tidak menghasilkan identitas JWT. Middleware lalu mencoba membaca username dari session. Username tersebut dicari kembali ke database; `user_id` dan role yang dipakai berasal dari database, bukan session.
4. **Tolak request tanpa identitas.** Jika session tidak berisi username atau pengguna session sudah tidak ditemukan, middleware mengembalikan `401 Unauthorized`.
5. **Periksa otorisasi role.** Middleware memanggil `UserRole::can_access` dengan role tepercaya, HTTP method request, dan path request. Query string tidak termasuk path karena yang dipakai adalah `request.uri().path()`.
6. **Teruskan request yang diizinkan.** Middleware menyimpan `AuthIdentity` pada request extensions, lalu memanggil `next.run(request)` agar handler berikutnya dapat memproses request.

### Perilaku fallback dan kegagalan

| Kondisi | Hasil |
| --- | --- |
| Tidak ada bearer JWT yang dapat diverifikasi, tetapi session valid | Autentikasi dilanjutkan menggunakan session. |
| JWT tidak cocok dengan pengguna/role/`token_version`, tetapi session valid | Autentikasi dilanjutkan menggunakan session. |
| JWT tidak menghasilkan identitas dan session tidak memiliki identitas | `401 Unauthorized`. |
| Pengguna session tidak ditemukan di database | `401 Unauthorized`. |
| Pengguna terautentikasi, tetapi role tidak boleh mengakses method/path | `403 Forbidden`. |
| Query database atau pembacaan session gagal | `500 Internal Server Error`; kegagalan dicatat di log server. |

Kegagalan verifikasi token diperlakukan sebagai tidak adanya identitas JWT sehingga session masih dapat dicoba. Namun, kegagalan operasi database setelah token lolos verifikasi, atau kegagalan membaca session, langsung menghasilkan `500`; kondisi tersebut tidak disamarkan sebagai login yang berhasil maupun sebagai `401`.

Respons error dibuat melalui [helper response](response.md) dalam format JSON:

```json
{
  "status": "error",
  "message": "Login diperlukan"
}
```

Nilai `message` menyesuaikan kondisi; status HTTP tetap menjadi pembeda utama antara autentikasi yang belum ada (`401`), role yang tidak diizinkan (`403`), dan kegagalan internal (`500`).

## Aturan akses role

Semua baris di bawah tetap mensyaratkan identitas yang valid. Matriks ini berlaku hanya pada route yang dipasang middleware.

| Role | Akses yang diizinkan |
| --- | --- |
| `admin` | Semua method pada route protected. |
| `buyer` | `GET /api/laptops` dan path yang diawali `/api/laptops/`. |
| `restocker` | `GET` pada path laptop tersebut dan `PATCH` pada path yang diawali `/api/laptops/`. |
| Semua role terautentikasi | `GET /api/auth/session` dan `POST /api/auth/logout`. |

Dengan demikian, `buyer` tidak dapat membuat, memperbarui, atau menghapus laptop. `restocker` dapat membaca dan memperbarui laptop, tetapi tidak dapat membuat atau menghapusnya. Role non-admin juga tidak dapat mengakses route admin. Aturan rinci beserta test-nya berada di [`role.rs`](../../../src/auth/role.rs).

## Integrasi dengan route dan handler

`routes.rs` memasang middleware melalui `middleware::from_fn_with_state(pool.clone(), auth::require_auth)` pada grup protected. `State` membawa pool database; router juga memasang `JwtService` sebagai `Extension` dan `SessionManagerLayer` untuk menyediakan session. `auth::require_auth` diekspor ulang dari `auth/mod.rs`, sehingga router tidak perlu mengimpor jalur modul internal.

Setelah lolos, handler dapat mengambil `AuthIdentity` dari request extensions untuk mengetahui pengguna yang sudah diverifikasi. Handler tidak perlu memeriksa ulang header JWT atau cookie session. Kegagalan terjadi sebelum handler dijalankan, sehingga handler protected hanya menerima request yang telah melewati autentikasi dan otorisasi.

## File terkait

- [JWT](jwt.md) menjelaskan format bearer, claims, expiry, dan verifikasi token.
- [Session](session.md) menjelaskan identitas session serta operasi login/logout.
- [Session dan JWT](sesi_jwt.md) membandingkan perilaku kedua metode autentikasi.
- [Routes](../routes.md) menjelaskan pemasangan middleware dan daftar endpoint.
- [Handler auth](../handlers/auth.md) menjelaskan login, logout, dan endpoint identitas saat ini.
