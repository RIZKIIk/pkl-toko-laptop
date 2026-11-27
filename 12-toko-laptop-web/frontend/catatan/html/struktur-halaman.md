# Struktur Halaman HTML

> **Navigasi:** [Pusat catatan](../README.md) · [Sintaks dasar](../02-sintaks-dasar.md)

**Buka source:** [index.html](../../index.html)

`index.html` adalah dokumen awal yang dikirim server ke browser. Konten login dan dashboard kemudian dibuat oleh JavaScript di elemen `#app`.

## Bagian dokumen

| Bagian | Fungsi |
| --- | --- |
| `<!doctype html>` | Memberi tahu browser bahwa dokumen memakai HTML modern. |
| `<html lang="id">` | Elemen akar dokumen; `lang` menyatakan bahasa utama agar pembaca layar dan browser dapat memilih pelafalan yang sesuai. |
| `<meta charset="utf-8">` | Menentukan encoding karakter agar huruf dan simbol tampil benar. |
| `viewport` | Membuat lebar halaman mengikuti perangkat ponsel. |
| `theme-color` | Warna antarmuka browser yang mengikuti tema aplikasi. |
| `description` dan `title` | Metadata ringkas serta judul tab awal. JavaScript memperbarui title saat halaman berganti. |
| `theme-init.js` | Script kecil biasa yang dijalankan sebelum stylesheet agar tema diterapkan lebih awal. |
| `<link rel="stylesheet">` | Memuat CSS. Urutan penting karena aturan CSS yang datang belakangan dapat menimpa aturan sebelumnya jika selector sama kuat. |
| `<script type="module">` | Memuat `app.js` sebagai ES module, sehingga bisa menggunakan `import` dan `export`. `defer` sudah menjadi perilaku default module script. |

## Elemen isi yang dipakai ulang

- `<main id="app" aria-live="polite">` adalah tempat JavaScript mengganti halaman login, daftar, dan dashboard. `aria-live` meminta teknologi bantu mengumumkan perubahan konten secara wajar.
- `#toast-region` adalah tempat pesan singkat ditambahkan oleh helper `toast()`. `aria-live="assertive"` membantu pesan penting segera diumumkan.
- `<dialog id="dialog">` dipakai untuk detail produk, tambah/ubah laptop, dan pembelian. Browser menyediakan perilaku modal dasar lewat `showModal()`.

Karena elemen global tersebut selalu ada, file JavaScript dapat mengambilnya satu kali lewat `document.querySelector()` di [shared.js](../js/shared.md).

## Mengapa konten tidak ditulis statis di HTML?

Dokumen HTML hanya menyediakan kerangka global. `app.js` membaca `location.pathname` untuk memilih fungsi `renderLogin()`, `renderRegister()`, atau `openDashboard()`. Halaman yang dipilih memasukkan markup ke `#app`, sedangkan URL API digunakan untuk mengambil data dinamis.

## Hubungan dengan CSS dan JavaScript

`/assets/css/base.css` dan file CSS berikutnya mengatur tampilan berdasarkan class dan ID pada markup. Path `/assets/...` adalah URL publik yang dipetakan backend melalui daftar aset yang diizinkan. Contohnya, SVG hero berada di `frontend/images/laptopku-hero.svg` dan disajikan sebagai `/assets/images/laptopku-hero.svg`. Script tema berada di `<head>`; entry point module juga diletakkan di sana, tetapi pemuatan module ditunda otomatis sampai dokumen selesai diparse.

