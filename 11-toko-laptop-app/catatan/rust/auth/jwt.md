# Materi Final: JWT pada `src/auth/jwt.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

![Ilustrasi: Anatomi JWT dan masa hidup](../../ilustrasi/halaman/08-jwt.svg "Anatomi JWT dan masa hidup")

[Bandingkan session dan JWT](sesi_jwt.md) · [Buka source `jwt.rs`](../../../src/auth/jwt.rs)

## 1. Tujuan modul

File `src/auth/jwt.rs` menyediakan mekanisme untuk:

1. menyimpan secret yang dipakai layanan JWT;
2. membuat access token setelah login berhasil;
3. memverifikasi token yang dikirim client;
4. mengambil token dari header HTTP `Authorization`.

Algoritma tanda tangan yang digunakan adalah **HS256**, yaitu HMAC dengan SHA-256. Secret yang sama digunakan untuk menandatangani dan memverifikasi token. Token berlaku selama **3.600 detik (satu jam)**.

Modul ini menangani format dan tanda tangan token. Modul ini **tidak** mencari akun ke database dan tidak menentukan izin route. Setelah JWT diverifikasi, [middleware autentikasi](middleware.md) mencocokkan klaim pengguna dengan database lalu memeriksa hak akses role.

## 2. Gambaran JWT

JWT adalah token yang umumnya terdiri atas tiga bagian yang dipisahkan titik:

```text
header.payload.signature
```

- **Header** menyatakan metadata token, termasuk algoritma tanda tangan.
- **Payload** memuat claims atau data tentang token dan penggunanya.
- **Signature** membantu mendeteksi perubahan pada header atau payload.

Bagian header dan payload dikodekan, bukan dienkripsi. Siapa pun yang memiliki token dapat membaca payload. Signature membantu memastikan data tidak diubah tanpa terdeteksi, tetapi tidak menyembunyikan isinya. Karena itu, jangan masukkan password, secret, atau data sensitif ke claims.

Pada aplikasi ini, prosesnya menggunakan secret simetris:

```text
claims + secret
      ↓ encode dengan HS256
 access token
      ↓ client mengirim token
decode + verifikasi dengan secret yang sama
      ↓
claims yang sudah diverifikasi
```

## 3. Isi claims

`Claims` adalah struct Rust yang diserialisasi sebagai payload ketika token diterbitkan dan di-deserialisasi kembali setelah token diverifikasi.

| Field | Tipe Rust | Kegunaan |
| --- | --- | --- |
| `sub` | `i64` | ID pengguna pemilik token. Nama `sub` adalah singkatan dari *subject*. Middleware memakai nilai ini untuk mencari pengguna di database. |
| `exp` | `u64` | Waktu kedaluwarsa dalam detik sejak Unix epoch (1 Januari 1970 UTC). Token ditolak setelah waktu ini terlewati. |
| `role` | `UserRole` | Role saat token dibuat. Middleware membandingkannya dengan role terbaru dari database sebelum menerima identitas JWT. |
| `token_version` | `i64` | Versi token pengguna saat JWT dibuat. Middleware membandingkannya dengan versi terbaru di database untuk menolak token yang telah dicabut. |

Contoh payload setelah di-decode:

```json
{
  "sub": 7,
  "exp": 1793864943,
  "role": "admin",
  "token_version": 0
}
```

Angka `exp` di atas hanya contoh timestamp; saat runtime nilainya dihitung dari waktu sekarang. `sub` dan `exp` merupakan nama klaim JWT yang lazim. `role` dan `token_version` adalah data khusus aplikasi. Role diserialisasi sebagai teks huruf kecil, seperti `"admin"`, sesuai konfigurasi serde pada [`UserRole`](../../../src/auth/role.rs).

### Perbedaan `role` dan `token_version`

- `role` mengingat role saat token diterbitkan. Bila role pengguna berubah, middleware mendapati nilai token dan database berbeda, lalu tidak menerima identitas dari JWT tersebut.
- `token_version` memungkinkan pencabutan banyak token sekaligus. Logout menaikkan versi di database. JWT lama pengguna membawa versi sebelumnya dan tidak lagi cocok, termasuk JWT yang belum mencapai `exp`.

Pencocokan database ini dilakukan oleh middleware pada request JWT. Karena itu, token proyek ini tidak hanya diterima berdasarkan signature dan expiry.

## 4. Alur penerbitan token

Setelah login memverifikasi username dan password, handler membaca ID, role, dan `token_version` pengguna dari database. Handler memanggil:

```rust
jwt.issue_token(user_id, role, token_version)
```

Di dalam `issue_token`, prosesnya adalah:

1. `SystemTime::now()` mengambil waktu sistem saat ini.
2. `duration_since(UNIX_EPOCH)?` mengubahnya menjadi durasi sejak Unix epoch. Operator `?` meneruskan error jika perhitungan gagal.
3. `.as_secs()` mengambil durasi sebagai jumlah detik.
4. `Claims { ... }` membentuk payload. Nilai expiry dihitung sebagai waktu sekarang ditambah `TOKEN_EXPIRED_SECONDS`.
5. `EncodingKey::from_secret(...)` membuat key encoding dari bytes secret.
6. `Header::new(Algorithm::HS256)` memilih algoritma tanda tangan.
7. `encode(...)` menserialisasi dan menandatangani claims.
8. Token yang berhasil dibentuk dibungkus menjadi `Ok(String)`.

Handler login mengembalikan token tersebut pada `data.access_token`, bersama tipe token dan masa berlaku. Rincian proses login ada di [handler auth](../handlers/auth.md).

## 5. Alur verifikasi token

Ketika middleware menerima request, ia mencoba mengambil bearer token dari header, lalu memanggil `verify_token`. Verifikasi meliputi:

1. Membuat `DecodingKey` dari secret layanan.
2. Membuat konfigurasi `Validation` yang membatasi algoritma ke HS256.
3. Mengatur `leeway` ke `0`, sehingga tidak ada toleransi tambahan untuk waktu expiry.
4. Memanggil `decode::<Claims>(...)`. Library memeriksa token sesuai konfigurasi, lalu mengubah payload ke `Claims`.
5. Mengembalikan klaim jika valid atau error jika token tidak dapat diverifikasi.

Validnya signature belum cukup untuk memastikan pengguna masih boleh mengakses endpoint. Middleware tetap mencocokkan pengguna, role, dan `token_version` dengan database, lalu memeriksa izin role. Lihat [catatan middleware](middleware.md).

## 6. Mengambil bearer token dari header

Client mengirim token pada header HTTP berikut:

```http
Authorization: Bearer <access-token>
```

`bearer_token` melakukan pemeriksaan berurutan:

1. `headers.get(AUTHORIZATION)` mencari header. Jika tidak ada, hasilnya `None`.
2. `.to_str()` mencoba membaca nilai header sebagai teks.
3. `.split_once(' ')` membagi pada spasi pertama menjadi skema dan isi token.
4. `scheme.eq_ignore_ascii_case("Bearer")` memastikan skema cocok tanpa membedakan kapitalisasi ASCII.
5. Token harus tidak kosong dan tidak boleh berisi whitespace tambahan.
6. Jika semua kondisi lolos, fungsi mengembalikan `Some(token)`; selain itu `None`.

Fungsi `claims_from_headers` menghubungkan ekstraksi header dengan verifikasi. Ia mengembalikan `Some(Claims)` jika keduanya berhasil dan `None` jika header tidak dapat dipakai atau token gagal diverifikasi. Bentuk ini memungkinkan middleware mencoba session sebagai jalur autentikasi berikutnya.

## 7. Pembahasan sintaks Rust di source

Bagian ini merangkum konstruksi sintaks yang muncul di `jwt.rs` dan menjelaskan perannya dalam file tersebut.

### 7.1 Dokumentasi, komentar, dan attribute

| Bentuk | Penjelasan |
| --- | --- |
| `//! ...` | Komentar dokumentasi untuk module yang sedang dibuka. Letaknya di awal file membuat keterangan berlaku untuk module JWT. |
| `/// ...` | Komentar dokumentasi untuk item tepat di bawahnya, misalnya konstanta, struct, field, atau method. |
| `// ...` | Komentar biasa untuk pembaca source; tidak menjadi bagian dari dokumentasi API. |
| `#[derive(Clone, Debug, Deserialize, Serialize)]` | Attribute meminta compiler atau macro menyediakan implementasi trait yang disebutkan. |
| `#[cfg(test)]` | Mengompilasi item di bawahnya hanya untuk konfigurasi test. |
| `#[test]` | Menandai fungsi sinkron sebagai satu unit test Rust. |

Attribute ditulis dengan `#[...]`; komentar dokumentasi menggunakan `///` atau `//!`. Keduanya memiliki fungsi berbeda.

### 7.2 `use`, path, dan pengelompokan import

Contoh dari source:

```rust
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
```

- `use` membawa nama ke scope saat ini agar tidak perlu selalu menulis path lengkap.
- `std` adalah standard library Rust.
- `::` memisahkan nama crate/module dan item di dalamnya.
- Kurung kurawal `{...}` mengelompokkan beberapa import.
- `time::{SystemTime, UNIX_EPOCH}` adalah pengelompokan bersarang: kedua nama berasal dari `std::time`.
- Koma memisahkan item import.

Import lainnya:

| Import | Kegunaan |
| --- | --- |
| `std::sync::Arc` | Membagikan secret secara aman di antara clone `JwtService`. |
| `std::time::{SystemTime, UNIX_EPOCH}` | Mengambil waktu sekarang dan menghitung timestamp expiry sejak Unix epoch. |
| `axum::http::{HeaderMap, header::AUTHORIZATION}` | Tipe kumpulan header HTTP dan nama header Authorization. |
| `jsonwebtoken::Algorithm` | Memilih algoritma tanda tangan HS256. |
| `jsonwebtoken::{EncodingKey, DecodingKey}` | Membentuk key dari secret untuk menerbitkan dan memverifikasi token. |
| `jsonwebtoken::{Header, Validation}` | Mengatur metadata algoritma saat encode dan aturan saat decode. |
| `jsonwebtoken::{encode, decode}` | Mengubah claims menjadi token bertanda tangan dan memverifikasi token menjadi claims. |
| `serde::{Deserialize, Serialize}` | Trait untuk membaca dan menghasilkan representasi data. |
| `super::role::UserRole` | `super` menunjuk module induk `auth`; `role::UserRole` mengambil enum role dari module saudara. |

### 7.3 Konstanta dan tipe data

```rust
pub const TOKEN_EXPIRED_SECONDS: u64 = 60 * 60;
```

- `pub` membuat item dapat diakses dari module lain.
- `const` mendeklarasikan nilai tetap.
- Nama konstanta menggunakan huruf besar dan underscore sesuai konvensi Rust.
- `: u64` menyatakan tipe bilangan bulat tanpa tanda 64-bit.
- `=` memberikan nilai.
- `60 * 60` menghitung 3.600 detik.
- `;` mengakhiri deklarasi.

Konstanta ini dipakai ketika membentuk nilai `exp`. Ia menyatakan masa berlaku, bukan timestamp expiry itu sendiri.

### 7.4 Struct, field, dan derive

```rust
#[derive(Clone)]
pub struct JwtService {
    secret: Arc<str>,
}
```

- `struct` mendefinisikan tipe gabungan dengan named fields.
- `JwtService` adalah nama tipe, ditulis PascalCase.
- `secret: Arc<str>` menyatakan field `secret` bertipe `Arc<str>`.
- Field tidak diberi `pub`, sehingga hanya module yang mendefinisikan `JwtService` dapat mengakses field itu secara langsung.
- `#[derive(Clone)]` menghasilkan implementasi trait `Clone`.

`Arc<T>` adalah pointer dengan reference count atomik. Di sini, `Arc<str>` memungkinkan beberapa salinan `JwtService` berbagi secret yang sama tanpa menyalin seluruh teks setiap kali service di-clone.

Constructor pada source mengisi field tersebut seperti berikut:

```rust
pub fn new(secret: String) -> Self {
    Self {
        secret: Arc::from(secret),
    }
}
```

`Self { ... }` membangun instance dari tipe yang sedang di-`impl`. `Arc::from(secret)` mengubah `String` yang diterima menjadi `Arc<str>`; nilai `secret` dipindahkan ke field, bukan dipinjam. Setelah dibuat, service dapat di-clone dengan berbagi alokasi `Arc` yang sama.

```rust
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: u64,
    pub role: UserRole,
    pub token_version: i64,
}
```

`Claims` mempunyai empat field publik. Tipe `i64` adalah bilangan bulat bertanda 64-bit; `u64` adalah bilangan bulat tanpa tanda 64-bit. `Serialize` diperlukan saat claims dikodekan menjadi payload token, sedangkan `Deserialize` diperlukan untuk membaca payload kembali sebagai `Claims`. `Debug` membantu inspeksi saat debugging dan `Clone` memungkinkan nilai claims disalin bila diperlukan.

### 7.5 `impl`, fungsi, method, dan `Self`

```rust
impl JwtService {
    pub fn new(secret: String) -> Self {
        // ...
    }
}
```

- `impl JwtService { ... }` mengelompokkan fungsi dan method yang berhubungan dengan tipe `JwtService`.
- `fn` mendeklarasikan fungsi.
- `pub fn` membuat fungsi dapat dipanggil dari module lain.
- `new` tidak menerima parameter `self`, sehingga merupakan associated function dan dipanggil dengan `JwtService::new(...)`.
- `Self` adalah nama tipe yang sedang di-`impl`, yaitu `JwtService`.
- `String` adalah teks dengan kepemilikan data sendiri. `new` menerima kepemilikan secret dari pemanggil.

Sebaliknya, `issue_token` dan `verify_token` menerima `&self`. Tanda `&` berarti meminjam instance untuk dibaca, tanpa memindahkan kepemilikannya. Pemanggil dapat memakai service kembali setelah method selesai.

### 7.6 Parameter, reference, dan nilai return

Signature berikut menunjukkan cara membaca bentuk fungsi:

```rust
pub fn issue_token(
    &self,
    user_id: i64,
    role: UserRole,
    token_version: i64,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
```

- `&self` meminjam service.
- Setiap parameter ditulis `nama: Tipe`.
- `->` memisahkan parameter dari tipe hasil.
- `Result<T, E>` menyatakan operasi dapat sukses dengan nilai `T` atau gagal dengan error `E`.
- `String` adalah token hasil ketika sukses.
- `Box<dyn Error + Send + Sync>` adalah error bertipe dinamis yang dibungkus `Box`; batas `Send + Sync` menyatakan error memenuhi trait agar aman digunakan dalam konteks lintas thread.
- `+` pada batas trait berarti error harus memenuhi semua trait tersebut.

Method verifikasi memiliki tipe hasil berbeda:

```rust
pub fn verify_token(
    &self,
    token: &str,
) -> Result<Claims, jsonwebtoken::errors::Error>
```

`&str` adalah pinjaman teks token. Jika berhasil, hasilnya `Claims`; jika gagal, error spesifik dari crate `jsonwebtoken`.

### 7.7 Membuat instance dan field shorthand

```rust
let claims = Claims {
    sub: user_id,
    exp: now + TOKEN_EXPIRED_SECONDS,
    role,
    token_version,
};
```

- `let` mengikat nilai ke nama lokal.
- `claims` adalah nama variable.
- `Claims { ... }` membangun instance struct.
- `sub: user_id` menyatakan field `sub` diisi dari variable `user_id`.
- `now + TOKEN_EXPIRED_SECONDS` menjumlahkan timestamp sekarang dengan durasi berlaku.
- `role` merupakan *field initialization shorthand*, singkatan dari `role: role`.
- `token_version` juga shorthand untuk `token_version: token_version`.
- Koma memisahkan field; titik koma mengakhiri statement.

Variable lokal immutable secara default. Kata `mut` hanya dibutuhkan bila binding itu sendiri akan diubah.

### 7.8 Waktu, konversi, dan operator `?`

```rust
let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)?
    .as_secs();
```

- `SystemTime::now()` memanggil associated function untuk membaca waktu sistem sekarang.
- `UNIX_EPOCH` adalah konstanta waktu awal Unix.
- `.duration_since(...)` memanggil method pada nilai sebelumnya dan menghasilkan `Result`.
- `?` mengambil nilai sukses atau langsung mengembalikan error dari fungsi saat ini.
- `.as_secs()` mengambil durasi dalam detik sebagai `u64`.
- Method chaining menghubungkan hasil satu pemanggilan ke pemanggilan berikutnya.

Pada `issue_token`, error waktu dapat diteruskan karena tipe hasil fungsi memuat error yang dapat dikonversi ke boxed error. Operator `?` tidak sama dengan `unwrap()`: `?` mengembalikan error kepada pemanggil, sedangkan `unwrap()` akan menghentikan eksekusi ketika menemukan error.

### 7.9 Key, encoding, dan peminjaman argumen

```rust
let key = EncodingKey::from_secret(self.secret.as_bytes());
let header = Header::new(Algorithm::HS256);
let token = encode(&header, &claims, &key)?;
```

- `EncodingKey::from_secret(...)` memanggil associated function untuk membentuk key dari bytes secret.
- `self.secret` membaca field service.
- `.as_bytes()` memberikan representasi byte yang dipinjam dari secret.
- `Header::new(Algorithm::HS256)` membentuk header dengan algoritma HS256.
- `&header`, `&claims`, dan `&key` meminjam nilai sebagai argumen, bukan memindahkan kepemilikannya.
- `encode` mengembalikan `Result`; `?` meneruskan error encoding.
- `let` pada contoh ini menghasilkan binding immutable.

Kode sesungguhnya membuat header langsung di dalam pemanggilan `encode`; variable `header` di atas hanya ditulis untuk memecah contoh agar lebih mudah dibaca.

Pada akhir fungsi, bentuk `Ok(encode(...)?)` memiliki dua lapis peran: `?` mengeluarkan token dari `Result` jika sukses atau meneruskan error, lalu `Ok(...)` menandai token sebagai hasil sukses dari `issue_token`.

### 7.10 `mut` dan konfigurasi validasi

```rust
let mut validation = Validation::new(Algorithm::HS256);
validation.leeway = 0;
```

- `Validation::new(...)` membuat konfigurasi validasi.
- `let mut` membuat binding yang boleh dimutasi.
- `validation.leeway = 0` mengubah field `leeway` menjadi nol detik.
- `0` dipakai sebagai nilai toleransi waktu; tidak ditambahkan toleransi ekstra pada pemeriksaan expiry.

Algoritma validasi dibatasi pada HS256 agar decoder memakai algoritma yang memang digunakan untuk penerbitan token.

### 7.11 Generic dan turbofish

```rust
decode::<Claims>(token, &key, &validation)
```

`decode` adalah fungsi generic: tipe data hasil deserialisasi ditentukan ketika fungsi dipanggil. Sintaks `::<Claims>` disebut *turbofish*. Bagian `Claims` memberi tahu compiler bahwa payload harus dibaca menjadi tipe `Claims`.

Hasil `decode` memuat data yang berhasil diverifikasi. `.claims` mengakses field claims dari hasil tersebut. `?` meneruskan error decode, kemudian `Ok(...)` mengembalikan claims yang valid dalam `Result`.

### 7.12 `HeaderMap`, `Option`, `Result`, dan pencocokan pola

Fungsi ekstraksi header:

```rust
pub fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let value = headers.get(AUTHORIZATION)?.to_str().ok()?;
    let (scheme, token) = value.split_once(' ')?;

    if scheme.eq_ignore_ascii_case("Bearer")
        && !token.is_empty()
        && !token.chars().any(char::is_whitespace)
    {
        Some(token)
    } else {
        None
    }
}
```

Penjelasan sintaks dan alurnya:

| Bentuk | Arti |
| --- | --- |
| `headers: &HeaderMap` | Meminjam kumpulan header untuk dibaca. |
| `-> Option<&str>` | Mengembalikan token yang dipinjam (`Some`) atau menyatakan tidak ada token yang dapat dipakai (`None`). |
| `headers.get(AUTHORIZATION)` | Menghasilkan `Option` karena header mungkin tidak tersedia. |
| `.to_str()` | Menghasilkan `Result` karena nilai header belum tentu dapat dibaca sebagai teks. |
| `.ok()` | Mengubah `Result` menjadi `Option`; detail error dibuang pada fungsi ekstraksi ini. |
| `?` pada rangkaian tersebut | Bekerja pada `Option`: jika salah satu langkah menghasilkan `None`, fungsi langsung mengembalikan `None`. |
| `split_once(' ')` | Membagi string pada spasi pertama, menghasilkan pasangan bagian sebelum dan sesudah spasi. |
| `let (scheme, token) = ...` | Destructuring tuple: dua bagian dipasang ke dua binding. |
| `if ... { ... } else { ... }` | Memilih nilai berdasarkan kondisi; kedua cabang menghasilkan `Option<&str>`. |
| `&&` | Operator AND: seluruh syarat harus benar. |
| `!` | Operator NOT: membalik hasil boolean. |
| `.chars().any(...)` | Mengiterasi karakter dan memeriksa apakah setidaknya satu karakter memenuhi predicate. |
| `char::is_whitespace` | Method karakter yang dipakai sebagai fungsi pemeriksa whitespace. |
| `Some(token)` / `None` | Varian enum `Option`, masing-masing menyatakan nilai tersedia atau tidak. |

Pemeriksaan menerima skema `Bearer` tanpa membedakan kapitalisasi ASCII. Token harus ada, tidak kosong, dan tidak mengandung whitespace. Token yang kembali adalah referensi ke bagian string header, bukan salinan `String`.

### 7.13 Mengubah `Result` menjadi `Option`

```rust
pub fn claims_from_headers(
    headers: &HeaderMap,
    jwt: &JwtService,
) -> Option<Claims> {
    jwt.verify_token(bearer_token(headers)?).ok()
}
```

- `headers: &HeaderMap` dan `jwt: &JwtService` dipinjam untuk dibaca.
- `bearer_token(headers)?` menghentikan fungsi dengan `None` bila header tidak berisi bearer token yang dapat dipakai.
- Jika token ada, `verify_token(...)` menghasilkan `Result<Claims, Error>`.
- `.ok()` mengubah `Ok(claims)` menjadi `Some(claims)` dan `Err(error)` menjadi `None`.

Dengan demikian, pemanggil fungsi ini tidak menerima detail penyebab kegagalan JWT. Pada alur aplikasi, middleware dapat melanjutkan pemeriksaan melalui session. Perbedaan umum tipe hasil:

| Tipe | Varian | Makna |
| --- | --- | --- |
| `Option<T>` | `Some(T)`, `None` | Nilai mungkin tersedia atau tidak; tidak membawa detail error. |
| `Result<T, E>` | `Ok(T)`, `Err(E)` | Operasi sukses atau gagal dengan informasi error. |

### 7.14 Module test, macro, dan assertion

Di bagian akhir file:

```rust
#[cfg(test)]
mod tests {
    use super::{JwtService, UserRole};

    const SECRET: &str = "test-secret-with-at-least-32-bytes-long";

    #[test]
    fn token_round_trip_returns_the_user_id_and_role() {
        // ...
    }
}
```

- `mod tests` membuat module anak bernama `tests`.
- `use super::{...}` mengambil nama dari module induk tempat test berada.
- `const SECRET: &str` membuat konstanta string yang dipinjam.
- `fn token_round_trip_returns_the_user_id_and_role()` mendeklarasikan fungsi test; nama deskriptif menjelaskan perilaku yang diuji.
- `#[cfg(test)]` membatasi kompilasi module untuk test.
- `#[test]` mendaftarkan fungsi sebagai unit test.

Macro assertion yang digunakan:

| Macro | Pemakaian di test |
| --- | --- |
| `assert_eq!(aktual, harapan)` | Memastikan claims hasil verifikasi sama dengan ID, role, atau versi yang diharapkan. |
| `assert!(kondisi)` | Memastikan kondisi bernilai benar, misalnya hasil verifikasi merupakan error. |
| `format!("{token}x")` | Membentuk string baru memakai format string dan menangkap variable `token` di dalam `{...}`. Test menambahkan karakter untuk mengubah token. |

`.to_owned()` membuat `String` yang dimiliki dari konstanta `&str`. `.unwrap()` mengambil nilai dari `Result` sukses pada test; jika hasilnya error, test gagal dengan panic. Penggunaan `unwrap()` di test membuat kegagalan cepat terlihat, sedangkan jalur aplikasi menggunakan `Result` agar error dapat ditangani.

Dua test yang tersedia memeriksa:

1. **Round trip:** token diterbitkan, diverifikasi, lalu `sub`, `role`, dan `token_version` diperiksa.
2. **Penolakan token:** token yang diubah dan token yang diverifikasi dengan secret berbeda harus menghasilkan error.

Test tersebut tidak menyatakan seluruh kemungkinan perilaku sudah diuji; contohnya, source ini tidak memiliki test khusus untuk token kedaluwarsa.

## 8. Sintaks lengkap berdasarkan bagian file

Untuk menghubungkan istilah di atas dengan struktur source, file dapat dibaca dalam empat bagian:

1. **Dokumentasi dan import:** komentar `//!`, `use`, path module, dan nama dependency.
2. **Konstanta dan model:** `TOKEN_EXPIRED_SECONDS`, `JwtService`, `Claims`, field, tipe, dan derive.
3. **Implementasi service:** `impl`, associated function `new`, method `issue_token`/`verify_token`, waktu, `Result`, key, encode, serta decode.
4. **Pembacaan header dan test:** `Option`, `?`, tuple, `if/else`, method chaining, attribute test, serta macro assertion.

Beberapa tanda baca Rust yang sering terlihat di file:

| Tanda | Fungsi |
| --- | --- |
| `::` | Memisahkan path module/item atau memanggil associated function, misalnya `JwtService::new`. |
| `.` | Mengakses field atau memanggil method, misalnya `self.secret.as_bytes()`. |
| `:` | Memisahkan nama dari tipe pada deklarasi, atau nama field dari nilainya pada struct. |
| `->` | Memisahkan parameter fungsi dari tipe return. |
| `&` | Meminjam nilai; `&mut` meminjam untuk dapat diubah. |
| `<...>` | Menyatakan parameter tipe generic, seperti `Arc<str>` atau `Result<T, E>`. |
| `?` | Meneruskan kegagalan dari `Result`/`Option` ke pemanggil fungsi. |
| `!` setelah nama macro | Memanggil macro, misalnya `assert!` atau `format!`. |
| `;` | Mengakhiri statement. Ekspresi terakhir dalam blok dapat menjadi nilai return tanpa `;`. |
| `{...}` | Membatasi body fungsi/module atau mengelompokkan nilai struct dan cabang blok. |
| `()` | Membungkus parameter fungsi dan pemanggilan fungsi. |
| `[]` pada attribute | Menulis konfigurasi compiler atau macro, misalnya `#[derive(...)]`. |

## 9. Contoh pemakaian dari client

Setelah login berhasil, salin nilai `data.access_token` dari response lalu kirim sebagai bearer token:

```bash
TOKEN='SALIN_ACCESS_TOKEN_DARI_RESPONSE_LOGIN'
curl -i \
  -H "Authorization: Bearer $TOKEN" \
  http://127.0.0.1:3001/api/laptops
```

Contoh tersebut menunjukkan header request, bukan kode Rust. Jangan menaruh token sungguhan ke catatan, repository, atau log. Cookie session dapat menjadi metode autentikasi alternatif; middleware mencoba session jika JWT tidak menghasilkan identitas yang valid.

## 10. Kondisi JWT tidak diterima

`verify_token` dapat mengembalikan error, antara lain ketika token tidak dapat di-decode, signature tidak cocok, algoritma tidak sesuai, atau token kedaluwarsa. `claims_from_headers` juga mengembalikan `None` jika header hilang atau format bearer tidak cocok.

Setelah JWT tidak menghasilkan identitas, middleware mencoba session. Jika session valid, request dapat tetap terautentikasi melalui session. Jika session juga tidak memberi identitas, middleware mengembalikan `401 Unauthorized`. Bila JWT lolos verifikasi, middleware masih harus memastikan pengguna ada dan `role` serta `token_version` cocok dengan database. Untuk detail hasil HTTP dan otorisasi, lihat [middleware](middleware.md).

## 11. Secret dan batas keamanan

Konfigurasi aplikasi membaca secret dari `JWT_SECRET` dan menolak secret yang panjangnya kurang dari 32 byte. Pemeriksaan konfigurasi berada di [config](../config.md); `JwtService::new` sendiri hanya menyimpan secret yang diberikan.

Hal yang perlu dijaga:

- Gunakan secret rahasia dan berbeda dari secret development/test.
- Jangan mencatat secret atau access token ke log.
- Jangan menganggap payload JWT terenkripsi.
- Jangan menerima role dalam payload JWT sebagai satu-satunya sumber izin; aplikasi mencocokkannya dengan database dan menjalankan pemeriksaan role.
- Hindari membagikan token karena pemegang token dapat menggunakannya selama token masih diterima aplikasi.

## 12. Hubungan dengan file lain

- [Handler auth](../handlers/auth.md) memanggil `issue_token` setelah kredensial login benar.
- [Middleware autentikasi](middleware.md) membaca dan memverifikasi JWT, memeriksa database, lalu mengotorisasi request.
- [Session](session.md) menjelaskan metode autentikasi cookie yang menjadi fallback.
- [Perbandingan session dan JWT](sesi_jwt.md) membahas perbedaan serta perilaku logout.
- [Role](../../../src/auth/role.rs) mendefinisikan enum `UserRole` yang dipakai claims.
- [Repository user](../repositories/user-repository.md) menyediakan identitas pengguna dan `token_version`.
- [Routes](../routes.md) memasang `JwtService` sebagai extension router.
- [Konfigurasi](../config.md) membaca serta memvalidasi `JWT_SECRET`.
