# Pusat Catatan Frontend LaptopKu

> **Navigasi:** [README proyek](../../README.md) · [Catatan backend](../../backend/catatan/README.md)

Folder ini menjelaskan cara kerja frontend LaptopKu dari struktur halaman sampai interaksi browser. Catatan dipisah berdasarkan HTML, CSS, JavaScript, dan file source supaya mudah dibuka sesuai topik.

## Mulai dari sini

1. Baca [gambaran frontend](01-gambaran-frontend.md) untuk mengenali arsitektur dan aliran data.
2. Pelajari [sintaks yang dipakai](02-sintaks-dasar.md) sebagai bekal membaca source.
3. Ikuti [struktur HTML](html/struktur-halaman.md), lalu buka [indeks CSS](css/README.md) dan [indeks JavaScript](js/README.md).

## Jalur belajar

| Tujuan | Urutan baca |
| --- | --- |
| Paham halaman yang tampil | [Gambaran frontend](01-gambaran-frontend.md) → [HTML](html/struktur-halaman.md) → [Dashboard](js/dashboard-view.md) |
| Paham tampilan | [Sintaks dasar](02-sintaks-dasar.md) → [CSS](css/README.md) → [tema gelap](css/theme.md) |
| Paham login dan akun | [HTML](html/struktur-halaman.md) → [shared.js](js/shared.md) → [login.js](js/login.md) → [register.js](js/register.md) |
| Paham data laptop | [dashboard.js](js/dashboard.md) → [dashboard-view.js](js/dashboard-view.md) → [laptop-actions.js](js/laptop-actions.md) |
| Paham menu samping | [Sidebar UI](css/dashboard.md) → [perilaku sidebar](js/sidebar.md) → [aturan layar kecil](css/responsive.md) |

## Peta source frontend

```text
index.html
├── theme-init.js     memilih tema sebelum halaman digambar
├── images/           ilustrasi SVG lokal
├── CSS               base → login → dashboard → components → responsive → theme
└── app.js             entry point dan penghubung event
    ├── shared.js      state, API, ikon, error, dan helper
    ├── login.js / register.js
    ├── dashboard.js / dashboard-view.js
    ├── laptop-actions.js
    ├── user-actions.js  form akun admin dan pilihan role dari API
    ├── sidebar.js
    └── theme.js
```

## Indeks catatan

| Bagian | Catatan |
| --- | --- |
| Dasar | [Gambaran frontend](01-gambaran-frontend.md) · [Sintaks dasar](02-sintaks-dasar.md) · [HTML](html/struktur-halaman.md) |
| CSS | [Indeks CSS](css/README.md) · [base.css](css/base.md) · [login.css](css/login.md) · [dashboard.css](css/dashboard.md) · [components.css](css/components.md) · [responsive.css](css/responsive.md) · [theme.css](css/theme.md) |
| JavaScript | [Indeks JavaScript](js/README.md) · [app.js](js/app.md) · [shared.js](js/shared.md) · [theme-init.js](js/theme-init.md) · [theme.js](js/theme.md) · [login.js](js/login.md) · [register.js](js/register.md) · [dashboard.js](js/dashboard.md) · [dashboard-view.js](js/dashboard-view.md) · [user-actions.js](js/user-actions.md) · [sidebar.js](js/sidebar.md) · [laptop-actions.js](js/laptop-actions.md) |

## Cara membaca catatan

- Tautan **Buka source** membuka file implementasi yang sedang dibahas.
- Nama seperti `state`, `api()`, `.sidebar`, dan `data-action` adalah nama yang benar-benar dipakai source.
- Potongan kode disederhanakan untuk menjelaskan satu konsep; source tetap menjadi acuan lengkap.
- Perubahan tampilan sering melibatkan tiga tempat: markup di JavaScript, aturan CSS, dan event di JavaScript.
- Backend mengatur validasi, izin, dan penyimpanan. Frontend membantu pengguna mengisi form dan menampilkan hasil, tetapi keputusan akses tetap harus diperiksa backend.

