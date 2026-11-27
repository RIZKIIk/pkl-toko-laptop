# Indeks Catatan CSS

> **Navigasi:** [Pusat frontend](../README.md) · [Sintaks dasar](../02-sintaks-dasar.md)

Urutan stylesheet di [index.html](../../index.html) menentukan bagaimana aturan dasar dan penyesuaian bertumpuk.

| Urutan | Source | Tanggung jawab |
| --- | --- | --- |
| 1 | [base.css](base.md) | Variabel warna, reset ringan, font, fokus keyboard, dan loader. |
| 2 | [login.css](login.md) | Halaman login/daftar, form, dan elemen brand. |
| 3 | [dashboard.css](dashboard.md) | Dashboard, navigasi samping, kartu, tabel, pencarian, dan status. |
| 4 | [components.css](components.md) | Dialog, toast, tombol tema, dan komponen global. |
| 5 | [responsive.css](responsive.md) | Penyesuaian tablet dan ponsel. |
| 6 | [theme.css](theme.md) | Override warna dan komponen untuk mode gelap. |

## Cara membaca selector

- `.sidebar` memilih semua elemen ber-class `sidebar`.
- `.sidebar .nav-link` berarti `.nav-link` yang berada di dalam `.sidebar`.
- `.nav-link.active` berarti satu elemen yang memiliki kedua class sekaligus.
- `:root` mewakili elemen akar dokumen; variabel yang ditulis di sana dapat dipakai banyak komponen.
- `:root[data-theme="dark"]` memilih dokumen ketika JavaScript mengatur `data-theme="dark"` pada elemen `<html>`.
- `@media (max-width: 620px)` mengaktifkan aturan saat viewport 620 piksel atau lebih kecil.

Untuk perubahan visual sidebar, periksa bersama-sama [dashboard.css](dashboard.md), [responsive.css](responsive.md), markup di [dashboard-view.js](../js/dashboard-view.md), dan perilaku di [sidebar.js](../js/sidebar.md).

