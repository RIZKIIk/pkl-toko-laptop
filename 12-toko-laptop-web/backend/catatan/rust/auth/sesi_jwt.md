# Perbedaan Session dan JWT

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Middleware autentikasi](middleware.md)

Panduan ini membandingkan dua cara aplikasi mengenali pengguna setelah login: session berbasis cookie dan JSON Web Token (JWT) berbasis bearer token. Keduanya tidak saling menggantikan dalam implementasi proyek ini: **satu login membuat session dan JWT sekaligus**, lalu client dapat mengautentikasi request dengan salah satunya.

## Gambaran singkat

- **Session:** informasi session disimpan di server. Client/browser menyimpan *session ID* di dalam cookie dan mengirim cookie itu pada request, sehingga server dapat menemukan session yang sesuai.
- **JWT:** informasi identitas (claims) dikemas menjadi token yang ditandatangani server. Client menyimpan token dan mengirimkannya melalui header `Authorization: Bearer <token>` pada request yang memerlukan autentikasi. Payload JWT dapat dibaca client; tanda tangan menjaga agar perubahan dapat dideteksi, bukan menyembunyikan isinya.

Keduanya tetap perlu dijaga sebagai kredensial. Siapa pun yang memperoleh cookie session atau JWT yang masih berlaku dapat mencoba menggunakannya sebagai pengguna tersebut.

## Perbandingan

| Aspek | Session | JWT |
| --- | --- | --- |
| Data yang dibawa client | Cookie berisi ID session, bukan seluruh identitas. | Token berisi ID pengguna (`sub`), waktu kedaluwarsa (`exp`), dan ID acak token (`jti`). |
| Penyimpanan identitas | Data session berada di Redis. Browser hanya membawa ID session dalam cookie. | JWT tetap dikirim client; Redis menyimpan registry `jti` dan ID akun dengan TTL, bukan bearer token mentah. |
| Cara dikirim | Browser mengirim cookie pada request sesuai aturan cookie. | Client mengirim `Authorization: Bearer <token>`. |
| Pemeriksaan di aplikasi | Middleware membaca username dari session lalu mencari pengguna dan role terkini di database. | Middleware memeriksa signature, expiry, registry Redis, lalu mencari pengguna dan role terkini di database. |
| Efek restart server | Session tetap tersedia selama Redis menyimpan record dan session belum kedaluwarsa. | Token hanya diterima jika registry Redis tetap tersedia dan ID token belum kedaluwarsa atau dicabut. |
| Perilaku saat logout | Session yang sedang dipakai dihapus dari store. Session lain tidak ikut terhapus. | ID JWT yang dipakai dihapus dari Redis, sehingga token langsung ditolak. |

> **Catatan penting:** JWT sering disebut membuat autentikasi *stateless* karena claims dibawa di token. Aplikasi ini tetap mengakses database setiap request JWT untuk memastikan pengguna masih ada dan mengambil role terkini untuk otorisasi.

## Alur login di proyek ini

1. Client mengirim username dan password ke endpoint login.
2. Setelah kredensial benar, handler mengambil ID pengguna dan role dari database.
3. Handler menerbitkan JWT satu jam, mendaftarkan `jti` di Redis, dan membuat session yang menyimpan username serta ID JWT. Session ID diganti setelah login.
4. Response login memberikan `access_token`; cookie session juga dikelola melalui mekanisme cookie.
5. Untuk request protected, middleware memeriksa signature, expiry, dan registry Redis JWT. Bearer JWT yang tidak valid atau sudah dicabut ditolak; tanpa bearer token, middleware dapat memakai session. Setelah identitas ditemukan, middleware memeriksa izin role untuk route yang diminta.

Detail klaim, expiry, dan signature ada di [catatan JWT](jwt.md). Detail cookie dan helper session ada di [catatan session](session.md); urutan validasi request dijelaskan di [catatan middleware](middleware.md).

## Contoh cara mengautentikasi request

### Memakai JWT

Client yang menyimpan `access_token` mengirimkannya sebagai bearer token:

```http
Authorization: Bearer <access_token>
```

### Memakai session

Browser mengirim cookie session yang diterima saat login secara otomatis sesuai aturan cookie. Untuk client non-browser, gunakan cookie jar atau kirim cookie yang diterima dari server. Contoh penggunaan tersedia di [panduan `curl`](../../curl.md).

Kedua cara tersebut menuju middleware autentikasi yang sama. Setelah identitas diverifikasi, handler protected menerima identitas pengguna yang sama bentuknya; detail mekanisme autentikasi tidak perlu diulang di setiap handler.

## Perbedaan perilaku saat logout

Logout mencabut token yang dipakai pada request dan membersihkan session aktif:

- **JWT:** ID token dihapus dari Redis. Bearer token yang sama langsung ditolak, meskipun JWT belum mencapai `exp`.
- **Session:** aplikasi menghapus session yang dipakai untuk request logout. Session di perangkat atau browser lain tidak otomatis dihapus.

Client tetap perlu menghapus salinan access token lokal setelah logout. Jika logout memakai session, ID JWT yang terhubung ke session ikut dicabut. Role yang berubah tetap mengubah izin request berikutnya karena middleware membaca role terkini dari database.

## Keterbatasan penyimpanan saat ini

Session dan registry JWT menggunakan Redis, sehingga beberapa instance backend dapat memakai data yang sama. Redis wajib tersedia saat aplikasi dimulai dan selama request protected dijalankan. JWT juga memerlukan secret yang sama dan terjaga untuk menerbitkan maupun memverifikasi token.

Lihat [konfigurasi](../config.md) untuk pengaturan secret JWT dan [routes](../routes.md) untuk pemasangan middleware serta session store.

## File terkait

- [JWT](jwt.md) menjelaskan claims dan penerbitan/verifikasi token.
- [Session](session.md) menjelaskan penyimpanan identitas dan helper cookie-session.
- [Middleware](middleware.md) menjelaskan cara request protected memilih JWT atau session.
- [Handler auth](../handlers/auth.md) menangani login dan logout.
- [Repository user](../repositories/user-repository.md) membaca identitas dan role akun.
