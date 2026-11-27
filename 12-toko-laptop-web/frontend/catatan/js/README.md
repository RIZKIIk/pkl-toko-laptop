# Indeks Catatan JavaScript

> **Navigasi:** [Pusat frontend](../README.md) · [Sintaks dasar](../02-sintaks-dasar.md)

JavaScript dibagi menurut tanggung jawab. Semua file memakai ES module; file dapat mengekspor fungsi dengan `export` dan mengambil fungsi lain dengan `import`.

| Urutan belajar | Source | Tanggung jawab |
| --- | --- | --- |
| 1 | [app.js](app.md) | Entry point: memilih halaman dan memasang listener global. |
| 2 | [shared.js](shared.md) | State, request API, token, ikon, error, toast, dan helper. |
| 3 | [theme-init.js](theme-init.md) · [theme.js](theme.md) | Menerapkan preferensi tema awal dan mengganti tema. |
| 4 | [login.js](login.md) · [register.js](register.md) | Menggambar serta mengirim form akun. |
| 5 | [dashboard.js](dashboard.md) · [dashboard-view.js](dashboard-view.md) | Memeriksa session, memuat produk, dan merender dashboard. |
| 6 | [laptop-actions.js](laptop-actions.md) | Operasi tambah, detail, ubah, hapus, dan beli produk. |
| 7 | [user-actions.js](user-actions.md) | Form admin membuat akun dengan pilihan role dari backend. |
| 8 | [sidebar.js](sidebar.md) | Navigasi aktif, sidebar ringkas, dan drawer ponsel. |

## Peta hubungan

```text
app.js ── memakai ── shared.js
  ├── login.js / register.js
  ├── dashboard.js ── dashboard-view.js
  │                       └── laptop-actions.js
  ├── user-actions.js ── API admin users/roles
  ├── sidebar.js
  └── theme.js ── shared.js

index.html → theme-init.js (lebih awal dari CSS)
```

Listener klik utama menggunakan `data-action` agar tetap berfungsi walau `#app` dirender ulang. Untuk alur browser ke backend, baca [gambaran frontend](../01-gambaran-frontend.md).

