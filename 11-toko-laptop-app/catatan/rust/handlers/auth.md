# `src/handlers/auth.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

[Buka kode sumber `auth.rs`](../../../src/handlers/auth.rs)

## Tanggung jawab

Handler ini memproses registrasi dan login, menampilkan identitas dari session/JWT, dan menghapus session saat logout. Akses database user, pemeriksaan password latihan, pembuatan JWT, response HTTP, dan middleware dipisahkan ke [modul auth](../auth/mod.md).

## Payload login

`LoginPayload` adalah struct yang menerima `username` dan `password` dari JSON untuk login dan registrasi. Derive `Deserialize` meminta Serde mengubah body JSON menjadi struct ini. Atribut `#[serde(deny_unknown_fields)]` menolak body dengan field tambahan.

## Endpoint

| Method | Path | Fungsi |
| --- | --- | --- |
| POST | `/api/auth/register` | Membuat akun dengan username unik. |
| POST | `/api/auth/login` | Memvalidasi kredensial, membuat session, dan menerbitkan JWT HS256 satu jam. |
| GET | `/api/auth/session` | Mengembalikan username dari session atau JWT aktif. |
| POST | `/api/auth/logout` | Menghapus data session di server; JWT tidak dicabut. |

## Fungsi dan alurnya

### `register(State(pool), Json(payload))`

1. Memangkas spasi pinggir username dan menolak username kosong atau lebih dari 100 karakter; password kosong juga ditolak dengan `400 Bad Request`.
2. Meminta auth service menyimpan akun baru. Username yang sudah ada menghasilkan `409 Conflict`; kegagalan database dicatat di server dan menghasilkan `500`.
3. Jika berhasil, mengembalikan `201 Created` dengan username. Registrasi tidak membuat session; pengguna perlu login secara terpisah.

### `login(State(pool), session, Json(payload))`

1. Memangkas spasi pinggir username dan menolak username kosong atau lebih dari 100 karakter; password kosong ditolak dengan `400 Bad Request`.
2. Memanggil `auth::verify_credentials` untuk mencari username di tabel `users` dan mencocokkan password latihan.
3. Mengembalikan `401 Unauthorized` jika username tidak ditemukan atau password tidak cocok. Kegagalan query dicatat di server dan menghasilkan `500`.
4. Jika cocok, menerbitkan JWT HS256 dengan subject username dan expiry satu jam.
5. Memanggil `session::start_session`. Fungsi itu mengganti ID session lalu menyimpan username di session server-side.
6. Mengembalikan JSON `200 OK` berisi username, `access_token`, `token_type`, dan `expires_in`. Cookie session dikelola oleh `SessionManagerLayer`, bukan dibuat manual oleh handler.

Untuk materi session, password registrasi disimpan langsung di database lokal dan dibandingkan secara langsung. Jangan gunakan implementasi ini untuk akun nyata atau produksi.

### `current_user(session)`

Mengembalikan username dari bearer JWT valid atau session aktif. Token tidak valid dan session yang tidak memiliki username menghasilkan `401`; kegagalan membaca session dicatat di server dan menghasilkan `500`.

### `logout(session)`

Memanggil `clear_session` untuk menghapus data session. JWT stateless tidak dapat dicabut oleh endpoint ini dan tetap valid sampai expiry satu jam, sehingga client perlu menghapus token setelah logout. Jika penghapusan session berhasil, handler mengembalikan `200 OK`; jika gagal, mencatat detail error di server dan mengembalikan `500`.

`login` menggunakan `State` untuk menerima pool database dan `Json` untuk membaca body. Dua fungsi lain hanya memerlukan `Session`; semuanya mengembalikan `Response` agar dapat menyusun status dan JSON yang berbeda pada tiap cabang.

`cycle_id()` mengganti ID session saat login untuk mengurangi risiko session fixation. Request tanpa session valid pada route yang dilindungi juga menghasilkan `401` melalui middleware.

Session cookie dikelola `tower-sessions`, dan klien seperti `curl` dapat menyimpan cookie dengan `-c cookies.txt` lalu mengirimkannya dengan `-b cookies.txt`. Data session disimpan di memori server dan hilang saat proses server dimulai ulang. Rincian implementasi ada di [session](../auth/session.md), [response auth](../auth/http.md), dan [repository user](../repositories/user-repository.md).
