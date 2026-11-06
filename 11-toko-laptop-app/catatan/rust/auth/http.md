# `src/auth/http.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

![Ilustrasi: Header auth membuka pintu](../../ilustrasi/halaman/07-http.svg "Header auth membuka pintu")


[Buka source](../../../src/auth/http.rs)

## Gambaran singkat

File ini menyediakan dua helper untuk membuat response HTTP autentikasi dalam format JSON:

| Helper | Dipakai saat | Status |
| --- | --- | --- |
| `error_response(status, message)` | Proses gagal | Ditentukan oleh pemanggil |
| `success_response(body)` | Proses berhasil | Selalu `200 OK` |

Kedua helper mengubah data yang diberikan menjadi `Response` Axum. Dengan begitu, handler tidak perlu menyusun response HTTP sendiri berulang kali. Keduanya adalah fungsi biasa, bukan `async`, karena hanya membentuk nilai di memori dan tidak menunggu I/O.

## Di halaman ini

[Import](#1-import) · [Response error](#2-helper-error_response) · [Response sukses](#3-helper-success_response) · [Alur](#alur-sederhananya)

## 1. Import

Bagian awal file:

```rust
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::{Value, json};
```

### Apa itu Axum?

Axum adalah library web untuk Rust. Aplikasi menggunakan Axum untuk menangani request HTTP dan mengirim response kembali ke client. Bagian ini mengimpor alat-alat Axum yang diperlukan untuk membuat response.

`use` berarti “bawa nama ini ke dalam cakupan file ini”. Setelah diimpor, kode dapat menulis `Json` daripada nama lengkap `axum::Json`.

| Nama | Kegunaan |
| --- | --- |
| `Json` | Membungkus data agar dikirim sebagai body JSON. Axum juga menyiapkan header `Content-Type` JSON. |
| `StatusCode` | Menyediakan kode HTTP seperti `200 OK`, `400 Bad Request`, dan `401 Unauthorized`. |
| `IntoResponse` | Trait (perilaku) yang menyediakan cara mengubah suatu nilai menjadi response HTTP Axum. |
| `Response` | Tipe response HTTP final yang dikembalikan oleh kedua helper. |
| `Value` | Tipe umum untuk menyimpan nilai JSON yang bentuknya bisa beragam. |
| `json!` | Macro dari `serde_json` untuk membuat nilai JSON dengan sintaks yang mudah dibaca. Tanda `!` menunjukkan bahwa ini macro. |

`serde_json` adalah library terpisah dari Axum. Di file ini, `json!` membuat datanya dan `Value` menjadi tipe data JSON-nya; Axum kemudian mengirimkannya melalui `Json`.

## 2. Helper `error_response`

Kode:

```rust
pub(crate) fn error_response(status: StatusCode, message: &'static str) -> Response {
    (
        status,
        Json::<Value>(json!({
            "status": "error",
            "message": message
        })),
    )
        .into_response()
}
```

### Deklarasi fungsi `error_response`

```rust
pub fn error_response(status: StatusCode, message: &'static str) -> Response
```

- `fn` menyatakan bahwa ini adalah fungsi.
- `error_response` adalah nama fungsi.
- `status: StatusCode` adalah parameter kode status HTTP. Pemanggil menentukan nilainya, misalnya `StatusCode::UNAUTHORIZED`.
- `message: &'static str` adalah parameter pesan berupa string yang dipinjam. `'static` berarti string itu dijamin tersedia selama program berjalan; string literal seperti `"Login diperlukan"` memenuhi syarat ini.
- `-> Response` menyatakan tipe hasil yang dikembalikan fungsi.

#### Visibilitas fungsi

- Tanpa `pub`, fungsi hanya dapat dipakai dari modul tempat fungsi itu didefinisikan.
- `pub` membuat fungsi ini dapat diakses melalui modul publik, termasuk dari crate pemanggil.
- Fungsi ini digunakan oleh middleware autentikasi dan handler; aturan visibilitas modul tetap berlaku, jadi `pub` pada fungsi saja tidak cukup jika modul induknya tidak diekspor.

`pub` membuat fungsi dapat diakses melalui modul publik oleh kode pemanggil di luar crate. Kode internal juga dapat menggunakannya.

### Cara membaca isi fungsi `error_response`

**1. `json!` menyusun data JSON**

```rust
json!({
    "status": "error",
    "message": message
})
```

Hasilnya adalah JSON dengan dua field. Nilai `message` berasal dari parameter fungsi. Jika pesannya `"Login diperlukan"`, hasil JSON-nya menjadi:

```json
{
  "status": "error",
  "message": "Login diperlukan"
}
```

**2. `Json::<Value>(...)` membungkus body**

`Json` menandai data tersebut sebagai body JSON untuk Axum. Bagian `::<Value>` disebut *turbofish*: cara Rust menuliskan tipe generik secara eksplisit. Di sini artinya, “isi `Json` ini bertipe `Value`”.

**3. `(status, Json(...))` memasangkan status dengan body**

Tanda kurung dan koma membentuk *tuple*, yaitu kelompok nilai berurutan. Tuple ini berisi:

1. Kode status HTTP, misalnya `401 Unauthorized`.
2. Body JSON yang akan dikirim.

**4. `.into_response()` mengubah tuple menjadi response HTTP**

Axum menyediakan implementasi trait `IntoResponse` untuk pasangan status dan body `Json`. Pemanggilan `.into_response()` menggunakan perilaku itu untuk mengubah tuple menjadi satu nilai bertipe `Response`.

Misalnya, secara konsep:

```rust
let response: Response = (
    StatusCode::UNAUTHORIZED,
    Json(json!({
        "status": "error",
        "message": "Login diperlukan"
    })),
)
    .into_response();
```

Hasilnya adalah response lengkap dengan status `401`, body JSON, dan header JSON yang sesuai. Tanpa `.into_response()`, nilainya masih berupa tuple, bukan `Response` yang diminta oleh deklarasi fungsi.

### 5. Nilai terakhir otomatis menjadi hasil fungsi

Ekspresi `.into_response()` tidak diakhiri titik koma (`;`), sehingga hasil ekspresinya langsung dikembalikan. Di Rust, kata `return` tidak wajib jika nilai yang dikembalikan adalah ekspresi terakhir dalam blok fungsi.

### Contoh pemakaian `error_response`

```rust
http::error_response(StatusCode::UNAUTHORIZED, "Login diperlukan")
```

Hasilnya: status `401 Unauthorized` dan body JSON berisi status `"error"` serta pesan `"Login diperlukan"`. Pesan yang dikirim ke client sebaiknya aman dan tidak berisi rincian internal seperti query database.

## 3. Helper `success_response`

Kode:

```rust
pub fn success_response(body: Value) -> Response {
    (StatusCode::OK, Json(body)).into_response()
}
```

### Deklarasi fungsi `success_response`

- `pub` membuat fungsi tersedia sebagai item publik pada modul ini.
- `fn success_response` mendeklarasikan fungsi bernama `success_response`.
- `body: Value` berarti pemanggil memberikan isi response dalam bentuk nilai JSON.
- `-> Response` berarti fungsi menghasilkan response HTTP Axum.

### Cara membaca isi fungsi `success_response`

1. `StatusCode::OK` memilih status `200 OK`. `OK` adalah konstanta pada `StatusCode`.
2. `Json(body)` membungkus nilai `body` sebagai body JSON. Tipe `Value` pada parameter sudah memberi tahu Rust tipe datanya, jadi `::<Value>` tidak perlu ditulis lagi.
3. `(StatusCode::OK, Json(body))` membentuk tuple status dan body.
4. `.into_response()` mengubah tuple tersebut menjadi `Response`.
5. Hasil `.into_response()` menjadi hasil fungsi karena tidak diakhiri titik koma.

### Contoh pemakaian `success_response`

```rust
http::success_response(json!({
    "message": "Berhasil login"
}))
```

Hasilnya adalah response `200 OK` dengan body:

```json
{
  "message": "Berhasil login"
}
```

Berbeda dari helper error, helper sukses selalu menetapkan status `200 OK`; pemanggil hanya memberikan isi JSON.

## Alur sederhananya

```text
data JSON + status HTTP
          ↓
       Json(...)
          ↓
  .into_response()
          ↓
   Response untuk client
```

Gunakan `error_response` jika perlu menentukan status error dan pesannya. Gunakan `success_response` jika proses berhasil dan statusnya `200 OK`.

## Sintaks Rust yang perlu diketahui

- `status: StatusCode` dan `body: Value` adalah parameter bernama beserta tipe datanya. Pemanggil harus memasok nilai yang cocok.
- `&'static str` adalah referensi teks yang berlaku selama program berjalan. Pesan literal seperti `"Login diperlukan"` memenuhi syarat tersebut dan tidak perlu dialokasikan sebagai `String`.
- `-> Response` menuliskan tipe hasil. Ekspresi terakhir tanpa `;` otomatis menjadi nilai hasil fungsi; jika diberi `;`, blok akan menghasilkan `()`, bukan `Response`.
- `(status, Json(...))` adalah tuple. Axum menyediakan implementasi trait `IntoResponse` untuk mengubah pasangan status dan body menjadi response lengkap.
- `Value` adalah nilai JSON yang dimiliki fungsi/pemanggil; `Json(body)` memindahkannya ke pembungkus response. Tidak ada peminjaman `&body` karena body perlu dimiliki response yang dikembalikan.
