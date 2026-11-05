# Panduan `curl` API

> **Navigasi:** [Indeks catatan](README.md) · [README proyek](../README.md) · [Endpoint API](../endpoin.md)

![Ilustrasi: Dari terminal ke endpoint](ilustrasi/halaman/05-curl.svg "Dari terminal ke endpoint")


Panduan ini berisi contoh untuk semua endpoint API toko laptop. Contoh memakai Bash/Linux/macOS dan URL lokal `http://127.0.0.1:3001` secara langsung. Jika host atau port server berubah, ganti URL tersebut pada command yang digunakan. Untuk Windows PowerShell, gunakan `curl.exe` bila perintah `curl` mengarah ke alias PowerShell.

## Di halaman ini

[Urutan login](#urutan-pemakaian) · [Semua contoh endpoint](#semua-endpoint) · [Opsi curl](#penjelasan-opsi-curl) · [Error umum](#error-umum)

## Urutan pemakaian

Endpoint laptop dan endpoint session aktif/logout memerlukan login terlebih dahulu. Pendaftaran publik membuat akun buyer. Untuk memilih role buyer atau restocker, login sebagai admin dari environment lalu buat akun melalui endpoint admin. Route terlindungi menerima cookie session atau JWT pada header `Authorization`.

Contoh login di bawah memakai placeholder. Ganti nilainya dengan `ADMIN_USERNAME` dan `ADMIN_PASSWORD` yang ada di `.env` lokal. Jangan menaruh password asli dalam dokumentasi, source code, atau file yang akan dibagikan. File cookie adalah kredensial aktif; jangan dibagikan atau di-commit.

```bash
curl -i \
  -c cookies.txt \
  -H 'Content-Type: application/json' \
  -d '{"username":"admin","password":"Rizki123!"}' \
  "http://127.0.0.1:3001/api/auth/login"
```

Response sukses login berstatus `200 OK`, mengatur cookie session, dan mengembalikan JWT pada `data.access_token` yang berlaku satu jam. Setelah request lain, cek identitas dengan `GET /api/auth/session`. Logout mencabut JWT yang dipakai pada request itu dan menghapus session aktif; hapus token lokal setelah logout.

## Semua endpoint

### 1. Health check — `GET /health`

Endpoint publik; tidak memerlukan login, body, atau `-X` karena curl secara default mengirim `GET` saat tidak diberi metode/body lain.

```bash
curl -i "http://127.0.0.1:3001/health"
```

Contoh response `200 OK`:

```json
{"status":"ok","service":"toko-laptop-app"}
```

### 2. Registrasi buyer — `POST /api/auth/register`

Endpoint publik yang membuat akun dengan role `buyer`. Username harus unik dan maksimal 100 karakter; password tidak boleh kosong. Role tidak bisa dipilih pada endpoint ini, dan registrasi tidak otomatis membuat session.

```bash
curl -i \
  -H 'Content-Type: application/json' \
  -d '{"username":"pengguna-baru","password":"password-lokal"}' \
  "http://127.0.0.1:3001/api/auth/register"
```

Response sukses berstatus `201 Created`. Username yang telah digunakan menghasilkan `409 Conflict`. Password di-hash dengan Argon2id sebelum disimpan.

### 3. Login admin — `POST /api/auth/login`

Endpoint publik. Untuk membuat akun dengan role tertentu, login memakai `ADMIN_USERNAME` dan `ADMIN_PASSWORD` dari environment server. `-c cookies.txt` menyimpan cookie dari response untuk dipakai pada request admin selanjutnya.

```bash
curl -i \
  -c cookies.txt \
  -H 'Content-Type: application/json' \
  -d '{"username":"ADMIN_USERNAME_DI_ENV","password":"PASSWORD_ANDA"}' \
  "http://127.0.0.1:3001/api/auth/login"
```

Contoh response sukses `200 OK`:

```json
{
  "status": "success",
  "message": "Login berhasil",
  "data": {
    "username": "admin",
    "role": "admin",
    "access_token": "<JWT>",
    "token_type": "Bearer",
    "expires_in": 3600
  }
}
```

Login gagal mengembalikan `401 Unauthorized`. Simpan/copy `access_token` dari response untuk request bearer. Login tidak memakai `-X POST`: opsi `-d` (`--data`) membuat curl otomatis memakai metode `POST`.

### 4. Membuat akun buyer/restocker — `POST /api/admin/users`

Wajib login sebagai admin. Body harus berisi `username`, `password`, dan `role`. Role yang bisa dibuat lewat endpoint ini adalah `buyer` atau `restocker`; role `admin` hanya dapat ditetapkan melalui konfigurasi server.

Contoh membuat akun buyer:

```bash
curl -i \
  -b cookies.txt \
  -H 'Content-Type: application/json' \
  -d '{"username":"pembeli-baru","password":"password-lokal","role":"buyer"}' \
  "http://127.0.0.1:3001/api/admin/users"
```

Contoh membuat akun restocker:

```bash
curl -i \
  -b cookies.txt \
  -H 'Content-Type: application/json' \
  -d '{"username":"petugas-stok","password":"password-lokal","role":"restocker"}' \
  "http://127.0.0.1:3001/api/admin/users"
```

Ganti username dan password contoh sesuai kebutuhan. Sukses mengembalikan `201 Created`; username yang sudah digunakan mengembalikan `409 Conflict`. Endpoint ini tidak menerima role admin.

### 5. Daftar laptop — `GET /api/laptops`

Wajib login. `-b cookies.txt` mengirim cookie yang sebelumnya disimpan saat login. Jika memilih JWT, ganti cookie dengan header `Authorization: Bearer $TOKEN`. Tidak perlu `-X GET`: tanpa opsi metode atau body, curl memakai `GET` secara default.

```bash
curl -i -b cookies.txt "http://127.0.0.1:3001/api/laptops"
```

Contoh response `200 OK`:

```json
{
  "status": "success",
  "data": [
    [1, "ThinkPad E14", "Lenovo", "Rp 12.500.000", 5]
  ]
}
```

Jika belum ada laptop, `data` adalah array kosong. Setiap item berbentuk `[id, nama, merek, harga, stok]`.

### 6. Tambah laptop — `POST /api/laptops`

Wajib login. `-H` menyatakan bahwa body yang dikirim adalah JSON. `-d` mengirim body dan otomatis memilih metode POST, jadi `-X POST` tidak diperlukan.

```bash
curl -i \
  -b cookies.txt \
  -H 'Content-Type: application/json' \
  -d '{"nama":"ThinkPad E14","merek":"Lenovo","harga":12500000,"stok":5}' \
  "http://127.0.0.1:3001/api/laptops"
```

Contoh response `201 Created`:

```json
{
  "status": "success",
  "message": "Laptop berhasil ditambahkan",
  "data": {
    "id": 1,
    "nama": "ThinkPad E14",
    "merek": "Lenovo",
    "harga": "Rp 12.500.000",
    "stok": 5
  }
}
```

Simpan nilai `data.id` dari response untuk detail, update, atau delete. Nama wajib maksimal 150 karakter; merek maksimal 100 karakter. Harga dan stok harus bilangan bulat nol atau lebih.

### 7. Detail laptop — `GET /api/laptops/{id}`

Ganti `1` dengan ID laptop yang didapat dari response tambah laptop. Wajib login. GET tidak memerlukan `-X`.

```bash
curl -i -b cookies.txt "http://127.0.0.1:3001/api/laptops/1"
```

Contoh response `200 OK`:

```json
{
  "status": "success",
  "data": [1, "ThinkPad E14", "Lenovo", "Rp 12.500.000", 5]
}
```

ID tidak valid menghasilkan `400 Bad Request`; ID positif yang tidak ditemukan menghasilkan `404 Not Found`.

### 8. Update sebagian field — `PATCH /api/laptops/{id}`

Wajib login. PATCH mengirim satu atau beberapa field yang ingin diubah; field lain tidak berubah. Di sini `-X PATCH` diperlukan karena `-d` sendiri secara default memilih POST, bukan PATCH.

```bash
curl -i \
  -b cookies.txt \
  -X PATCH \
  -H 'Content-Type: application/json' \
  -d '{"nama":"ThinkPad E14 Gen 5","harga":13500000}' \
  "http://127.0.0.1:3001/api/laptops/1"
```

Contoh response `200 OK`:

```json
{
  "status": "success",
  "message": "Laptop berhasil diperbarui",
  "data": [1, "ThinkPad E14 Gen 5", "Lenovo", "Rp 13.500.000", 5]
}
```

Body tanpa field yang dapat diubah ditolak dengan `400 Bad Request`.

### 9. Hapus laptop — `DELETE /api/laptops/{id}`

Wajib login. Karena tidak ada body, `-X DELETE` dipakai untuk memilih metode HTTP DELETE.

```bash
curl -i \
  -b cookies.txt \
  -X DELETE \
  "http://127.0.0.1:3001/api/laptops/1"
```

Contoh response `200 OK`:

```json
{
  "status": "success",
  "message": "Laptop berhasil dihapus",
  "id": 1
}
```

Jika ID tidak ditemukan, response `404 Not Found`.

### 10. Lihat session aktif — `GET /api/auth/session`

Wajib login. Endpoint ini mengembalikan username dan role akun yang terautentikasi. GET tidak memerlukan `-X`.

Dengan cookie session:

```bash
curl -i -b cookies.txt "http://127.0.0.1:3001/api/auth/session"
```

Dengan JWT bearer, isi `TOKEN` dengan nilai `data.access_token` dari response login:

```bash
TOKEN='tempel_jwt_di_sini'

curl -i \
  -H "Authorization: Bearer $TOKEN" \
  "http://127.0.0.1:3001/api/auth/session"
```

Contoh response `200 OK`:

```json
{
  "status": "success",
  "data": {
    "username": "admin",
    "role": "admin"
  }
}
```

JWT digunakan untuk autentikasi. Server mengambil role terbaru pengguna dari database, jadi role tidak perlu dibaca dari token. Tanpa cookie session atau bearer JWT yang valid, server mengembalikan `401 Unauthorized`.

### 11. Logout — `POST /api/auth/logout`

Wajib memakai cookie session atau JWT bearer yang valid. Tidak ada body, maka metode ditentukan eksplisit memakai `-X POST`. Logout mencabut JWT bearer yang dipakai pada request itu dan menghapus session aktif.

```bash
curl -i \
  -b cookies.txt \
  -c cookies.txt \
  -X POST \
  "http://127.0.0.1:3001/api/auth/logout"
```

Contoh response `200 OK`:

```json
{
  "status": "success",
  "message": "Logout berhasil"
}
```

Sesudah logout, endpoint yang dilindungi kembali menghasilkan `401` sampai login lagi.

## Penjelasan opsi curl

| Opsi | Arti | Alasan digunakan |
| --- | --- | --- |
| `-i` | Tampilkan header response bersama body. | Memudahkan melihat status HTTP serta `Set-Cookie` saat login. |
| `-H 'Content-Type: application/json'` | Menambahkan header request. | Memberi tahu Axum bahwa request body adalah JSON. |
| `-d '...'` | Mengirim data/body request. | Dipakai pada login dan create; curl otomatis mengubah metode menjadi POST. |
| `-X PATCH` | Memaksa metode HTTP tertentu. | Diperlukan untuk PATCH karena curl tidak dapat menyimpulkan PATCH hanya dari `-d`. |
| `-X DELETE` | Memaksa metode HTTP DELETE. | Dipakai untuk delete yang tidak mempunyai body. |
| `-X POST` | Memaksa metode HTTP POST. | Dipakai untuk logout karena tidak mengirim body; kalau memakai `-d`, `-X POST` bisa dihilangkan. |
| `-b cookies.txt` | Membaca cookie dari file dan mengirimkannya. | Mempertahankan session pada request protected. |
| `-c cookies.txt` | Menulis cookie dari response ke file. | Menyimpan cookie yang server berikan saat login; juga memperbarui cookie setelah logout. |
| `-X GET` | Memaksa metode GET. | Tidak dipakai karena curl sudah menggunakan GET secara default saat tidak ada `-d` atau metode lain. |
| URL lengkap | Menentukan alamat server dan path endpoint secara langsung. | Contoh memakai `http://127.0.0.1:3001`; ubah URL pada command jika host atau port server berbeda. |

Catatan tentang `-X`: opsi ini hanya memilih/menimpa nama metode HTTP; ia tidak otomatis menambahkan body atau header JSON. Gunakan bersama `-H` dan/atau `-d` sesuai kebutuhan. Hindari menambahkan `-X GET` pada contoh GET karena tidak menambah fungsi.

## Error umum

| Status | Penyebab umum | Yang perlu diperiksa |
| --- | --- | --- |
| `400` | Body JSON atau data laptop tidak valid; ID bukan bilangan positif. | Nama/merek, angka harga/stok, dan format JSON. |
| `401` | Login belum dilakukan, kredensial salah, atau session/JWT tidak valid. | Login ulang; kirim `-b cookies.txt` untuk cookie atau header `Authorization: Bearer $TOKEN` untuk JWT. |
| `404` | Laptop dengan ID tersebut tidak ada. | Periksa ID dari response create. |
| `500` | Query database atau session store gagal. | Log server; detail internal tidak dikirim pada response API. |

Jika Axum menolak JSON yang malformed atau tipe konten yang salah sebelum handler berjalan, bentuk response error dapat berbeda dari JSON error aplikasi.

## Ilustrasi alur request

```text
register (sekali) → login → simpan cookie/JWT
                           ↓
                    CRUD laptop
                           ↓
                       logout
```
