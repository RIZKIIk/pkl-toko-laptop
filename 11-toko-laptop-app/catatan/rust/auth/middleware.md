# `src/auth/middleware.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

[Buka source `middleware.rs`](../../../src/auth/middleware.rs)

## Tanggung jawab

`require_auth` mengautentikasi request memakai JWT bearer atau session, memeriksa hak akses role, lalu meneruskan request yang diizinkan. Router menerapkan middleware ini hanya ke route protected melalui `auth::require_auth`.

`AuthIdentity` menyimpan `user_id`, `username`, dan role yang telah diverifikasi. Middleware menambahkannya ke request sehingga handler protected dapat memakai identitas tepercaya tanpa mengulang pemeriksaan.

## Alur pemeriksaan

1. Middleware mencoba membaca dan memverifikasi bearer JWT melalui `jwt::claims_from_headers`.
2. Jika claims valid, middleware mencari pengguna dari `sub` dan memastikan role serta `token_version` sama dengan nilai di database.
3. Jika JWT tidak menghasilkan identitas, middleware membaca session melalui `session::session_identity`, lalu memeriksa username di database. Role selalu diambil dari database.
4. Request tanpa identitas menerima `401`. Kegagalan database atau session menghasilkan `500` dan dicatat di log server.
5. `UserRole::can_access` memeriksa role terhadap HTTP method dan path. Identitas yang tidak diizinkan menerima `403`.
6. Untuk akses yang sah, middleware memasukkan `AuthIdentity` ke request lalu menjalankan `next.run(request)`.

## Integrasi

`State<PgPool>` menyediakan koneksi database, `Extension<JwtService>` menyediakan layanan JWT, `Session` memberi akses ke session request, dan `Next` meneruskan request ke handler berikutnya. Response error dibuat melalui [helper response](response.md).

`auth::require_auth` tetap diekspor ulang oleh `auth/mod.rs`, sehingga pemasangan middleware di `routes.rs` dan pemanggil lain tidak perlu memakai jalur modul yang berbeda.

Aturan akses berada pada [`UserRole::can_access`](../../../src/auth/role.rs). Konfigurasi route protected dan session store berada di [routes](../routes.md).
