# Gambaran Frontend

> **Navigasi:** [Pusat catatan](README.md) · [Backend](../../backend/catatan/README.md)

**Buka source:** [index.html](../index.html) · [app.js](../js/app.js) · [shared.js](../js/shared.js)

Frontend LaptopKu adalah aplikasi browser yang disajikan backend Rust. Browser mengambil satu dokumen HTML, lalu modul JavaScript memilih halaman login, daftar, atau dashboard dan meminta data melalui API.

## Urutan halaman dimulai

1. Browser membaca `index.html` dan menyiapkan elemen `#app`, area notifikasi, serta elemen dialog.
2. `theme-init.js` membaca pilihan tema sebelum CSS digambar, sehingga halaman tidak perlu berkedip dari terang ke gelap.
3. File CSS dimuat. `base.css` menyediakan fondasi; file berikutnya menambah halaman akun, dashboard, komponen, aturan layar, dan tema.
4. `app.js` berjalan sebagai ES module, menghubungkan aksi UI, lalu memeriksa alamat saat ini.
5. Jika alamatnya `/dashboard`, aplikasi memeriksa session, mengambil data laptop, lalu meminta `dashboard-view.js` membentuk markup.

```text
Browser → index.html → theme-init.js → CSS → app.js
                                           ├─ /login      → login.js
                                           ├─ /register   → register.js
                                           └─ /dashboard  → dashboard.js → API Rust
                                                                ↓
                                                        dashboard-view.js
```

## Pembagian tanggung jawab

| File/kelompok | Tugas |
| --- | --- |
| `index.html` | Kerangka dokumen, elemen global, urutan stylesheet, dan entry point. |
| `css/*.css` | Warna, ukuran, tata letak, tema, dan penyesuaian ukuran layar. |
| `js/app.js` | Memilih halaman dan meneruskan klik, input, serta perubahan kontrol. |
| `js/shared.js` | State bersama, request API, penanganan error, ikon, escape HTML, dan toast. |
| `js/login.js`, `js/register.js` | Form masuk dan daftar. |
| `js/dashboard.js`, `js/dashboard-view.js` | Memeriksa session, mengambil daftar laptop, dan merender dashboard. |
| `js/laptop-actions.js` | Dialog detail, tambah, ubah, hapus, dan pembelian laptop. |
| `js/user-actions.js` | Form admin membuat akun dengan pilihan role dari API. |
| `js/sidebar.js` | Menu ringkas desktop dan drawer navigasi ponsel. |
| `js/theme-init.js`, `js/theme.js` | Memilih dan mengganti tema terang/gelap. |

## Satu alur data: membuka dashboard

`app.js` memanggil `openDashboard()`. Fungsi itu mengirim `GET /api/auth/session`. Jika session tersedia, data pengguna masuk ke `state.user`, lalu `loadLaptops()` meminta `GET /api/laptops`. Hasilnya dinormalisasi dan ditaruh di `state.laptops`. Terakhir `renderDashboard()` membentuk HTML menggunakan state tersebut.

Saat pengguna mencari, memfilter, atau mengurutkan produk, browser mengolah daftar yang sudah dimuat. Untuk detail atau perubahan produk, `laptop-actions.js` mengirim request baru ke backend. Jadi tabel dapat diperbarui tanpa memuat ulang seluruh dokumen.

## Session dan token di browser

Helper `api()` mengirim cookie dengan `credentials: "include"` dan, bila ada, token dari `sessionStorage` di header `Authorization`. Token dihapus dari penyimpanan tab ketika session tidak sah atau pengguna keluar. Tema memakai `localStorage` karena pilihan tampilannya boleh bertahan setelah browser ditutup. Rincian token ada di [shared.js](js/shared.md), sedangkan validasi akun berada di [backend](../../backend/catatan/README.md).

## Batas tugas frontend

Frontend bisa menyembunyikan tombol yang tidak sesuai role agar antarmuka lebih jelas. Namun, frontend bukan penjaga keamanan: backend tetap harus memeriksa session, role, dan validitas data pada setiap endpoint.

