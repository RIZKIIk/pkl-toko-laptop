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
| `jti` | ID acak unik untuk token ini. | Memungkinkan logout mencabut satu JWT. |

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

`JwtService` menyimpan secret sebagai `Arc<str>`. `Arc` memungkinkan service di-clone dengan biaya rendah saat dipakai bersama oleh handler dan middleware. Secret tidak dimasukkan ke claims atau response. `Claims` adalah bentuk data yang diserialisasi ke payload JWT. Role diserialisasi sebagai teks huruf kecil, misalnya `"admin"`.

Di bagian atas source, `//!` adalah dokumentasi untuk module, `///` mendokumentasikan item tepat di bawahnya, dan `//` adalah komentar biasa untuk pembaca kode. Bentuk `#[...]` adalah attribute yang memberi instruksi tambahan ke compiler.

## Sintaks Rust yang digunakan

Bagian ini menjelaskan konstruksi yang muncul di source, dari import sampai test.

### Import dan tipe

| Sintaks | Penjelasan |
| --- | --- |
| `use std::{ sync::Arc, time::{SystemTime, UNIX_EPOCH} };` | `use` membawa nama dari module agar dapat dipakai tanpa menulis path penuh. `{...}` mengelompokkan beberapa nama; kelompok bersarang mengambil nama waktu dari `std::time`. |
| `use axum::http::{HeaderMap, header::AUTHORIZATION};` | Mengimpor tipe map header HTTP dan konstanta nama header `Authorization`. `::` memisahkan nama module dan item. |
| `use super::role::UserRole;` | `super` menunjuk module induk dari `auth::jwt`; `role::UserRole` mengambil tipe role dari sibling module `role`. |
| `impl JwtService { ... }` | `impl` mengelompokkan fungsi yang terkait dengan tipe. `new` adalah associated function karena tidak menerima `self`; method seperti `issue_token` dan `verify_token` menerima `&self` dan dipanggil pada instance. |
| `Arc<str>` | `Arc` adalah pointer dengan hitungan referensi atomik yang memungkinkan beberapa pemilik berbagi data. `<str>` adalah parameter tipe; `str` di sini disimpan sebagai teks yang dibagi melalui `Arc`. `Arc::from(secret)` mengubah `String` menjadi bentuk `Arc<str>` yang dibagi. |
| `HeaderMap`, `HeaderValue`, `&str` | `HeaderMap` memuat header request. `HeaderValue` adalah nilai header. `&str` adalah pinjaman teks, jadi fungsi dapat membaca string tanpa mengambil kepemilikannya. |
| `Result<T, E>` dan `Option<T>` | `Result` menyatakan sukses (`Ok(T)`) atau gagal (`Err(E)`). `Option` menyatakan nilai ada (`Some(T)`) atau tidak ada (`None`). |
| `;` dan ekspresi terakhir | Titik koma mengakhiri statement. Ekspresi terakhir pada body fungsi tanpa titik koma menjadi nilai yang dikembalikan; contohnya `Ok(...)` di akhir `new` bukan statement terpisah. |

### Konstanta, struct, dan method

| Sintaks | Penjelasan |
| --- | --- |
| `pub const TOKEN_EXPIRED_SECONDS: u64 = 60 * 60;` | `pub` membuat item bisa diakses dari module lain. `const` adalah nilai tetap saat kompilasi. `u64` adalah bilangan bulat tanpa tanda 64-bit; `60 * 60` menghasilkan 3.600 detik. |
| `#[derive(Clone)]` | Attribute meminta compiler membuat implementasi `Clone`, agar service dapat diduplikasi. Karena secret berada di `Arc`, clone hanya menambah hitungan pemilik, bukan menyalin seluruh secret. |
| `pub struct JwtService { ... }` | `struct` mendefinisikan tipe yang menyimpan data. Field `secret` tidak diberi `pub`, jadi hanya module ini yang dapat membacanya langsung. |
| `#[derive(Clone, Debug, Deserialize, Serialize)]` | Membuat implementasi trait secara otomatis. `Serialize` mengubah `Claims` menjadi payload token; `Deserialize` membentuk `Claims` dari payload. `Debug` membantu menampilkan nilai saat debugging dan `Clone` mengizinkan penyalinan nilai claims. |
| `pub struct Claims { pub sub: i64, ... }` | Setiap field menyimpan satu claim. `i64` menyimpan ID pengguna, `u64` menyimpan waktu expiry, `UserRole` menyimpan role, dan `String` menyimpan `jti`. `pub` pada field memungkinkan module lain membaca field tersebut. |
| `pub fn new(secret: String) -> Self` | `fn` mendefinisikan fungsi. `String` adalah teks yang dimiliki pemanggil; `Self` berarti tipe yang sedang diimplementasikan, yaitu `JwtService`. |
| `&self`, `&value`, `&mut value` | `&self` meminjam instance service selama method berjalan. `&value` meminjam nilai untuk dibaca; `&mut value` meminjamnya agar dapat diubah. Referensi mencegah fungsi mengambil kepemilikan nilai yang dipinjam. |
| `Result<String, Box<dyn Error + Send + Sync>>` | Method mengembalikan token sebagai `String` atau error. `dyn Error` adalah trait object untuk beberapa jenis error; `Box` menyimpannya di heap. Batas `Send + Sync` membuat error aman dipindahkan dan dibagi antar thread. |

### Menerbitkan token dengan `issue_token`

1. `let now = ...` membuat binding lokal. `SystemTime::now()` mengambil waktu saat ini; `.duration_since(UNIX_EPOCH)` mengubahnya menjadi durasi sejak 1 Januari 1970 UTC; `.as_secs()` mengambil jumlah detik sebagai `u64`.
2. `?` setelah `duration_since` mengembalikan error dari `issue_token` jika jam sistem berada sebelum Unix epoch. Jika berhasil, operator itu mengambil nilai sukses dari `Result` agar dapat dipakai berikutnya.
3. `let mut token_id = [0_u8; 32];` membuat array 32 byte yang diawali nol. `mut` mengizinkan isinya diubah. `0_u8` adalah literal angka nol bertipe byte (`u8`).
4. `let mut rng = OsRng;` membuat generator angka acak yang menggunakan sumber acak sistem operasi. `RngCore` adalah trait yang menyediakan method `try_fill_bytes`; `try_` berarti pengisian bisa gagal dan error-nya diteruskan dengan `?`.
5. `token_id.iter()` meminjam byte satu per satu. `.map(|byte| ...)` menjalankan closure untuk tiap byte; `|byte|` menuliskan parameter closure. `format!("{byte:02x}")` membuat dua digit heksadesimal. `{byte}` mengambil nilai dari variabel bernama sama, `02` meminta lebar minimal dua karakter, dan `x` memilih huruf heksadesimal kecil. `.collect()` menggabungkan potongan itu menjadi `String`, sehingga 32 byte menjadi 64 karakter. `: String` memberi tahu compiler tipe hasil yang dikumpulkan.
6. `Claims { sub: user_id, exp: now + TOKEN_EXPIRED_SECONDS, role, jti }` membuat nilai struct. `role` dan `jti` memakai field initialization shorthand karena nama field sama dengan nama binding. `now + TOKEN_EXPIRED_SECONDS` membuat expiry satu jam setelah waktu saat ini.
7. `EncodingKey::from_secret(self.secret.as_bytes())` membuat key HS256 dari byte secret. `.as_bytes()` memberi tampilan byte dari teks tanpa membuat string baru.
8. `Header::new(Algorithm::HS256)` membuat header JWT yang menyatakan algoritma HMAC-SHA256. `encode(&header, &claims, &key)` menserialisasi claims dan menandatangani token.
9. `Ok(encode(...)?)` membungkus token sukses dalam `Ok`. `?` di dalamnya meneruskan error encoding; `Box` pada tipe hasil memungkinkan error library dikembalikan sebagai error method ini.

### Memverifikasi token dengan `verify_token`

1. `DecodingKey::from_secret(...)` membuat key untuk memeriksa signature dengan secret yang sama.
2. `let mut validation = Validation::new(Algorithm::HS256);` membuat aturan verifikasi khusus HS256. `mut` diperlukan karena baris berikut mengubah aturan.
3. `validation.leeway = 0;` menetapkan toleransi waktu expiry menjadi nol detik.
4. `decode::<Claims>(token, &key, &validation)` memanggil fungsi generic `decode` dengan tipe hasil `Claims`. Sintaks `::<Claims>` disebut turbofish; compiler diminta mengubah payload ke struct itu.
5. `.claims` mengambil claims dari hasil decode. `?` meneruskan error jika format, signature, algoritma, atau expiry tidak valid; `Ok(...)` membungkus claims yang valid sebagai hasil sukses.

### Membaca header bearer

| Sintaks | Penjelasan |
| --- | --- |
| `headers.get(AUTHORIZATION)` | Mencari nilai header `Authorization`; hasilnya `Option` karena header bisa tidak ada. |
| `.to_str().ok()?` | Mengubah nilai header menjadi teks. `to_str()` menghasilkan `Result`; `.ok()` mengubahnya menjadi `Option`, lalu `?` mengembalikan `None` jika nilainya bukan teks yang valid. |
| `split_once(' ')` | Memecah teks pada spasi pertama; hasilnya pasangan `(scheme, token)`. `?` mengembalikan `None` jika spasi tidak ditemukan. |
| `scheme.eq_ignore_ascii_case("Bearer")` | Membandingkan skema tanpa membedakan huruf besar/kecil ASCII, jadi `Bearer` dan `bearer` diterima. |
| `&&` dan `!` | `&&` berarti semua syarat harus benar. `!` membalik nilai boolean; kondisi ini menolak token kosong dan token yang masih mengandung whitespace. |
| `token.chars().any(char::is_whitespace)` | Memeriksa apakah ada karakter whitespace di token. `char::is_whitespace` dipakai sebagai fungsi pemeriksa untuk setiap karakter. |
| `Some(token)` / `None` | Mengembalikan token jika skema dan isinya benar; selain itu fungsi mengembalikan `None`. `if` dan `else` memilih hasil berdasarkan syarat boolean. |
| `(scheme, token)` | Pola tuple yang menerima dua bagian hasil `split_once`: teks sebelum spasi dan teks setelah spasi. |
| `jwt.verify_token(bearer_token(headers)?).ok()` | Mengekstrak bearer token; `?` berhenti jika tidak ada. Lalu token diverifikasi. `.ok()` mengubah `Result<Claims, Error>` menjadi `Option<Claims>`, sehingga error token diperlakukan sebagai tidak ada claims valid. |

Pada signature `bearer_token(headers: &HeaderMap) -> Option<&str>`, tipe `Option<&str>` berarti hasilnya dapat berupa pinjaman teks token atau `None`. Rust menyimpulkan masa pinjaman teks berasal dari `headers`, sehingga token tidak dapat dipakai setelah header asalnya hilang.

### Test di bagian bawah file

`#[cfg(test)]` menyertakan module hanya ketika test dibangun. `mod tests { ... }` membuat module anak khusus test, dan `use super::{JwtService, UserRole};` mengambil nama dari module JWT induk. `#[test]` menandai fungsi test biasa.

`SECRET` adalah konstanta secret khusus test. `.to_owned()` membuat `String` milik sendiri dari `&str`. `.unwrap()` mengambil nilai sukses dan menghentikan test jika hasilnya error. `assert_eq!` membandingkan dua nilai; `assert!(...is_err())` memastikan operasi menghasilkan error. `format!("{token}x")` membuat token yang diubah dengan menambah huruf `x`, agar test memastikan signature yang berubah ditolak. `other_jwt` memakai secret berbeda untuk memastikan token tidak dapat diverifikasi dengan key lain.

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
- [Source repository JWT](../../../src/repositories/jwt_repository.rs) menyimpan `jti` yang dicabut; [migration 0007](../database/0007-revoke-jwts.md) membuat tabelnya.
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
