# `src/auth/session.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

![Ilustrasi siklus session cookie](../../ilustrasi/halaman/52-auth-session.svg "Cookie client menunjuk ke session di Redis")

[Bandingkan session dan JWT](sesi_jwt.md)

[Buka source `session.rs`](../../../src/auth/session.rs)

## Tanggung jawab

File ini khusus mengelola session login melalui `tower-sessions`. Pemeriksaan JWT, autentikasi request, dan otorisasi role berada di [middleware](middleware.md).

| Item | Tugas |
| --- | --- |
| `SESSION_USERNAME_KEY` | Menyamakan nama key username dalam session. |
| `SESSION_JWT_ID_KEY` | Menyimpan ID JWT yang perlu dicabut ketika session logout. |
| `SessionIdentity` | Membungkus username dan ID JWT dari session. |
| `mulai_sesi` | Mengganti ID session setelah login lalu menyimpan username dan ID JWT. |
| `identitas_sesi` | Membaca username serta ID JWT dari session request. |
| `hapus_sesi` | Menghapus data session aktif dari store. |

## Helper session

### `mulai_sesi(session, username, jwt_id)`

Fungsi ini memanggil `session.cycle_id()` sebelum menyimpan username menggunakan `SESSION_USERNAME_KEY` dan ID JWT menggunakan `SESSION_JWT_ID_KEY`. Mengganti ID session setelah login membantu mencegah session pra-login dipakai kembali. Error dari operasi session diteruskan sebagai `tower_sessions::session::Error`.

### `identitas_sesi(session)`

Fungsi asinkron ini mengembalikan `Result<Option<SessionIdentity>, Error>`:

- `Ok(None)` berarti belum ada username di session.
- `Ok(Some(identity))` berarti username ditemukan.
- `Err(error)` berarti terjadi kegagalan saat membaca session.

Username dari session belum membuktikan bahwa akun masih ada. Middleware memeriksanya lagi ke database sebelum meneruskan request.

### `hapus_sesi(session)`

Memanggil `session.flush()` untuk menghapus session aktif dari store. Handler logout membaca ID JWT yang terhubung dengan session, lalu mencabutnya melalui registry Redis sebelum session dibersihkan.

## Penyimpanan session

Router memakai `RedisStore`. Session tidak bergantung pada memori proses backend dan bisa dibagikan oleh instance yang memakai Redis yang sama. Registry JWT juga memakai koneksi Redis tersebut; ID token dan ID akun memiliki TTL sesuai waktu `exp`. Logout mencabut JWT yang dipakai dan session yang aktif. Session lain pada perangkat berbeda tetap aktif.

Contoh login dan request dengan cookie tersedia di [panduan curl](../../curl.md). File cookie berisi kredensial aktif; jangan dibagikan atau dimasukkan ke version control.

## File terkait

- [Middleware](middleware.md) memeriksa identitas session/JWT dan izin role.
- [Handler auth](../handlers/auth.md) memulai session setelah login dan menghapusnya saat logout.
- [Routes](../routes.md) memasang `SessionManagerLayer`.
- Source memiliki unit test untuk menyimpan, membaca, dan membersihkan session.
