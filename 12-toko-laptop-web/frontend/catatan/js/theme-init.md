# `theme-init.js` — Tema Sebelum Halaman Digambar

> **Navigasi:** [Indeks JavaScript](README.md) · [Source](../../js/theme-init.js) · [Tombol tema](theme.md) · [CSS tema](../css/theme.md)

File ini dijalankan sebagai script biasa dari `<head>` di `index.html`, sebelum stylesheet, untuk mencegah halaman tampil sebentar dengan tema yang salah.

## Sintaks dan langkah

- `(() => { ... })()` adalah **IIFE**: fungsi langsung dibuat lalu dijalankan sekali. Variabel di dalamnya tidak bocor ke scope global.
- `try/catch` melindungi pembacaan `localStorage` jika akses storage dibatasi browser.
- Nilai tersimpan diterima hanya bila sama dengan `"light"` atau `"dark"`; jika tidak, `matchMedia("(prefers-color-scheme: dark)")` membaca pilihan sistem.
- Ternary `kondisi ? nilaiA : nilaiB` memilih tema yang akan dipakai.
- `document.documentElement.dataset.theme = theme` mengisi atribut `data-theme` pada `<html>`.
- `?.setAttribute(...)` memperbarui meta theme-color hanya jika elemen meta ditemukan.

Setelah tema awal diterapkan, [theme.css](../css/theme.md) memakai attribute selector untuk memilih palette yang sesuai. [theme.js](theme.md) menangani perubahan tema setelah tombol ditekan.

