# `src/auth/jwt.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

![Ilustrasi: Anatomi JWT dan masa hidup](../../ilustrasi/halaman/08-jwt.svg "Anatomi JWT dan masa hidup")


[Buka kode sumber `jwt.rs`](../../../src/auth/jwt.rs)

## Tujuan

Modul ini menerbitkan dan memverifikasi access token JWT menggunakan HMAC-SHA256 (HS256). Token memberi client cara mengautentikasi request tanpa mengirim password lagi. Secret dibaca dari `JWT_SECRET`, yang harus berisi sedikitnya 32 byte.

> **Ringkas:** login membuat JWT satu jam dengan ID pengguna (`sub`) dan `role`; route terlindungi memeriksa signature, expiry, pengguna, dan role terbaru.

## Isi token

JWT aplikasi membawa empat klaim (*claims*):

| Klaim | Nilai | Kegunaan |
| --- | --- | --- |
| `sub` | ID pengguna berupa angka. | Menyatakan identitas pemilik token (*subject*). |
| `exp` | Waktu kedaluwarsa sebagai Unix timestamp dalam detik. | Menentukan kapan token tidak berlaku lagi. |
| `role` | Role pengguna, misalnya `admin`. | Menentukan akses yang dimiliki token; middleware mencocokkannya dengan role terbaru di database. |
| `jti` | ID acak unik untuk token ini. | Memungkinkan logout mencabut satu JWT tanpa `auth_version`. |

Contoh payload setelah di-decode:

```json
{
  "sub": 7,
  "exp": 1793864943,
  "role": "admin",
  "jti": "ID_ACAK_UNIK"
}
```

Payload JWT ditandatangani, bukan dienkripsi. Client dapat membaca isinya. Jangan masukkan password, secret, atau data sensitif ke dalam claims.

## Cara kerja di aplikasi

1. Saat startup, `main.rs` memberikan `JWT_SECRET` ke `create_router`, lalu router membuat `JwtService`.
2. Setelah kredensial login cocok, handler mengambil ID dan role pengguna dari database. `issue_token(user_id, role)` menyimpan keduanya bersama klaim expiry satu jam.
3. `jsonwebtoken::encode` menandatangani claims dengan algoritma HS256 dan secret aplikasi.
4. Client mengirim token melalui header `Authorization: Bearer <access_token>`.
5. `bearer_token` membaca header dan mengambil token setelah skema `Bearer`. Header dengan format lain dianggap tidak memiliki bearer token.
6. `verify_token` memeriksa algoritma, signature, dan expiry. Middleware menolak `jti` yang tercatat di tabel `revoked_jwts`, lalu mencari pengguna berdasarkan `sub` dan mencocokkan `role` dengan role terbaru di database.

`JwtService` menyimpan secret sebagai `Arc<str>`. `Arc` memungkinkan service di-clone dengan biaya rendah saat dipakai bersama oleh handler dan middleware. Secret tidak dimasukkan ke claims atau response.

## Sintaks Rust yang digunakan

| Sintaks | Penjelasan |
| --- | --- |
| `pub const TOKEN_EXPIRED_SECONDS: u64 = 60 * 60;` | `const` adalah nilai tetap saat kompilasi; `u64` adalah bilangan bulat tanpa tanda 64-bit. Perkalian menghasilkan 3.600 detik. |
| `#[derive(Clone)]` | Attribute meminta compiler membuat implementasi `Clone`, sehingga `JwtService` bisa diduplikasi untuk dibagikan ke request. `Arc` membuat duplikasi secret murah. |
| `struct Claims { ... }` | Struct mendefinisikan bentuk data token. `#[derive(Deserialize, Serialize)]` memungkinkan library JWT membaca claims dan menulisnya ke token. |
| `&self`, `String` | `&self` meminjam service tanpa mengambil alihnya; `String` memiliki teks secret yang diberikan saat service dibuat. |
| `Result<String, Error>` | Fungsi dapat berhasil dengan `String` atau gagal dengan error. `Box<dyn Error + Send + Sync>` menyatukan berbagai error yang dapat dikirim dan dibagi antar thread. |
| `?` | Mengembalikan error lebih awal jika operasi gagal; jika sukses, mengambil nilai di dalam `Result`. Di sini dipakai untuk waktu sistem, encoding token, dan decoding claims. |
| `Option<&str>` | Hasil ekstraksi header bisa memiliki token (`Some`) atau tidak (`None`). Operator `?` pada `Option` menghentikan parsing jika header tidak ada atau formatnya keliru. |
| `split_once(' ')` | Memecah header pada spasi pertama menjadi skema dan isi token. `eq_ignore_ascii_case` menerima variasi kapitalisasi `Bearer`. |
| `#[cfg(test)]` dan `#[test]` | Bagian pertama hanya dikompilasi untuk test; attribute kedua menandai fungsi sebagai test. Test memeriksa token valid, token yang diubah, serta secret yang berbeda. |

Method chain seperti `Validation::new(...); validation.leeway = 0;` membuat aturan verifikasi lalu mengatur toleransi expiry menjadi nol. `decode::<Claims>(...)` memakai parameter tipe generik `Claims`, sehingga payload hasil decode dibentuk sebagai struct tersebut.

## Contoh request

Login mengembalikan token pada `data.access_token`. Salin nilainya dan sertakan sebagai bearer token:

```bash
TOKEN='SALIN_ACCESS_TOKEN_DARI_RESPONSE_LOGIN'
curl -i -H "Authorization: Bearer $TOKEN" \
  http://127.0.0.1:3001/api/laptops
```

JWT dan cookie session dapat dipakai sendiri-sendiri. Middleware memeriksa JWT lebih dulu, lalu mencoba session jika token tidak ada atau tidak valid. Lihat [middleware session](session.md) untuk urutan lengkap.

## Kapan token ditolak?

Token tidak memberi akses jika header tidak ada atau salah format, isi token berubah, token ditandatangani dengan secret lain, algoritmanya bukan HS256, token sudah kedaluwarsa atau dicabut, pengguna tidak ditemukan, atau role di token berbeda dari role terbaru di database. Middleware menghentikan request dan mengirim `401 Unauthorized`.

## Logout dan pencabutan token

JWT tidak membawa `auth_version`. Setiap token punya `jti` acak; logout menyimpan `jti` JWT yang dipakai ke tabel `revoked_jwts`, sehingga token itu langsung ditolak sampai kedaluwarsa. Session cookie aktif juga dihapus. Token lain milik pengguna tidak ikut dicabut. Client tetap perlu menghapus token lokal setelah logout.

## File terkait

- [Handler auth](../handlers/auth.md) menerbitkan token setelah login berhasil.
- [Middleware session](session.md) memeriksa token sebelum meneruskan request protected.
- [Routes](../routes.md) memasang service JWT sebagai extension router.
- [Konfigurasi](../config.md) memeriksa panjang secret dari environment.

Source juga memiliki unit test untuk penerbitan/verifikasi token serta penolakan token yang diubah atau ditandatangani dengan secret berbeda.

## Ilustrasi perjalanan JWT

```text
user ID + role + waktu expiry
        ↓ sign dengan secret HS256
     access token
        ↓ Authorization: Bearer ...
verify signature + expiry + user ID + role
        ├─ valid → username dan role terbaru
        └─ gagal → 401
```
