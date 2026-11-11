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
| Data yang dibawa client | Cookie berisi ID session, bukan seluruh identitas. | Token berisi claims seperti ID pengguna (`sub`), expiry (`exp`), role, dan `token_version`. |
| Penyimpanan identitas | Data session berada di server. Pada proyek ini session disimpan di `MemoryStore`. | Claims berada di payload token yang ditandatangani. Payload dapat dibaca client, tetapi perubahan tanpa secret membuat signature tidak valid. |
| Cara dikirim | Browser mengirim cookie pada request sesuai aturan cookie. | Client mengirim `Authorization: Bearer <token>`. |
| Pemeriksaan di aplikasi | Middleware membaca username dari session lalu mencari pengguna dan role terkini di database. | Middleware memeriksa signature dan expiry, lalu mencari pengguna di database dan mencocokkan role serta `token_version`. |
| Efek restart server | Session dalam `MemoryStore` hilang saat server dimulai ulang. | Token tetap tersimpan pada client sampai kedaluwarsa, tetapi tetap perlu lolos validasi signature, pengguna, role, dan versi token. |
| Pencabutan saat logout | Session yang sedang dipakai dihapus dari store. Session lain tidak ikut terhapus. | `token_version` pengguna dinaikkan, sehingga seluruh JWT lama pengguna ditolak pada request berikutnya. |

> **Catatan penting:** JWT sering disebut membuat autentikasi *stateless* karena claims dibawa di token. Namun, aplikasi ini tetap mengakses database setiap request JWT untuk memastikan pengguna masih ada dan role serta `token_version` masih cocok. Jadi, jangan menganggap pemeriksaan JWT di proyek ini hanya memerlukan verifikasi signature.

## Alur login di proyek ini

1. Client mengirim username dan password ke endpoint login.
2. Setelah kredensial benar, handler mengambil ID pengguna, role, dan `token_version` dari database.
3. Handler menerbitkan JWT satu jam dan membuat session yang menyimpan username. Session ID diganti setelah login.
4. Response login memberikan `access_token`; cookie session juga dikelola melalui mekanisme cookie.
5. Untuk request protected, middleware mencoba JWT lebih dulu. Jika JWT tidak menghasilkan identitas valid, middleware mencoba session. Setelah identitas ditemukan, middleware memeriksa izin role untuk route yang diminta.

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

Logout memiliki dampak berbeda pada kedua mekanisme:

- **JWT:** aplikasi menaikkan `users.token_version`. Semua JWT yang membawa versi sebelumnya menjadi tidak valid, termasuk token yang belum kedaluwarsa dan token dari perangkat lain.
- **Session:** aplikasi menghapus session yang dipakai untuk request logout. Session di perangkat atau browser lain tidak otomatis dihapus.

Karena login membuat kedua kredensial, client sebaiknya menghapus access token lokal dan cookie terkait setelah logout. Session di perangkat lain tidak dicabut oleh kenaikan `token_version`; JWT lama di perangkat tersebut ditolak, tetapi session-nya masih dapat dikenali sampai session itu dihapus atau store hilang.

Penjelasan lebih rinci tentang pencabutan tersedia di [catatan `token_version`](../../token_ver.md).

## Keterbatasan penyimpanan saat ini

Session menggunakan `MemoryStore`, sehingga session hilang ketika proses server dimulai ulang dan tidak dibagikan antar-instance server. Ini sesuai untuk pengembangan sederhana, tetapi tidak cocok untuk deployment multi-instance atau kebutuhan session yang harus bertahan melewati restart. JWT juga memerlukan secret yang sama dan terjaga untuk menerbitkan maupun memverifikasi token.

Lihat [konfigurasi](../config.md) untuk pengaturan secret JWT dan [routes](../routes.md) untuk pemasangan middleware serta session store.

## File terkait

- [JWT](jwt.md) menjelaskan claims dan penerbitan/verifikasi token.
- [Session](session.md) menjelaskan penyimpanan identitas dan helper cookie-session.
- [Middleware](middleware.md) menjelaskan cara request protected memilih JWT atau session.
- [Handler auth](../handlers/auth.md) menangani login dan logout.
- [Repository user](../repositories/user-repository.md) membaca identitas serta mengubah `token_version`.
