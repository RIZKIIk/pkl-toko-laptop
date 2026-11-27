# `shared.js` — State dan Helper Bersama

> **Navigasi:** [Indeks JavaScript](README.md) · [Source](../../js/shared.js) · [Alur frontend](../01-gambaran-frontend.md)

Modul ini mengekspor elemen global, state, SVG, error API, wrapper `fetch`, serta fungsi yang dipakai beberapa halaman.

## State aplikasi

`state` adalah object singleton yang menyimpan pengguna, daftar laptop, pencarian, filter, urutan, dan keadaan sidebar desktop. Modul lain mengimpor object yang sama sehingga perubahan state dapat dibaca oleh view.

`app` dan `dialog` adalah referensi DOM yang diambil memakai `document.querySelector()`. `icons` menyimpan template string SVG agar ikon konsisten dan tidak perlu disalin di banyak file.

## Token dan penyimpanan browser

`TOKEN_KEY` adalah nama kunci `sessionStorage`. `token()` membacanya; `saveToken(value)` menyimpan token jika ada atau menghapusnya jika nilainya kosong. `sessionStorage` hanya berlaku untuk tab/session browser saat ini. Cookie session dikirim terpisah oleh browser karena wrapper API memakai `credentials: "include"`.

## Helper teks dan role

- `esc(value)` mengubah karakter `&`, `<`, `>`, tanda kutip, dan apostrof menjadi HTML entity sebelum nilai dinamis dimasukkan ke template markup.
- `roleName(role)` memetakan `admin`, `buyer`, dan `restocker` ke nama tampilan; `||` menyediakan label cadangan.
- `html = String.raw` adalah tag template literal yang mempertahankan isi string mentah. Ia **bukan sanitizer**; nilai dari user/API tetap harus melalui `esc()` saat disisipkan ke HTML.

## API dan penanganan gagal

`ApiError extends Error` menyimpan `status` HTTP. Fungsi `api(path, options = {})`:

1. Membuat `Headers` dan menambahkan `Content-Type` JSON jika ada body.
2. Menambahkan `Authorization: Bearer ...` ketika token tersimpan.
3. Memanggil `fetch()` dengan opsi yang diberikan serta `credentials: "include"`.
4. Mengubah kegagalan jaringan menjadi `ApiError` berstatus `0`.
5. Membaca JSON response, menangani `401`, lalu melempar `ApiError` bila `response.ok` bernilai false.
6. Mengembalikan body ketika request berhasil.

`options.auth === false` menandai request seperti login dan daftar yang belum punya autentikasi. `...options` adalah object spread: properti opsi disalin sebagai bagian dari objek konfigurasi `fetch`.

## Toast, error, dan brand

`toast()` membuat `<div>`, mengisi pesan lewat `textContent` (bukan `innerHTML`), menambahkannya ke region notifikasi, lalu menghapusnya setelah timer. `reportError()` memilih pesan akses ditolak untuk status `403`, atau pesan error umum. `brand()` mengembalikan markup logo yang digunakan ulang.

