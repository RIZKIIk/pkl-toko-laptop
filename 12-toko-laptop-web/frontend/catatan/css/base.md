# `base.css` — Fondasi Visual

> **Navigasi:** [Indeks CSS](README.md) · [Source](../../css/base.css)

File ini memuat nilai global yang dipakai komponen lain.

## Sintaks dan bagian utama

- `:root { --ink: ... }` mendeklarasikan custom properties warna, bayangan, dan font. `var(--ink)` mengambil nilai tersebut.
- `* { box-sizing: border-box; }` membuat lebar elemen mencakup padding dan border, agar perhitungan layout lebih mudah.
- `body` menetapkan margin awal, ukuran minimum, latar, dan warna teks global.
- Selector gabungan `button, input, select` menyamakan font kontrol form dengan font halaman.
- `:focus-visible` memberi garis fokus yang terlihat ketika navigasi menggunakan keyboard. Ini membantu pengguna mengetahui posisi fokus.
- `scroll-margin-top` memberi jarak saat browser melompat ke bagian dashboard melalui anchor.
- `.boot-screen` memakai Flexbox (`display`, `align-items`, `justify-content`) untuk menempatkan indikator loading di tengah.
- `@keyframes spin` menjelaskan animasi putaran loader; `animation` menerapkan animasi itu.
- `@media (prefers-reduced-motion: reduce)` mengurangi durasi animasi bagi pengguna yang memilih gerakan minim di pengaturan perangkat.

## Contoh variabel

```css
:root { --soft: #f3f6f6; }
body { background: var(--soft); }
```

Nilai tema gelap menimpa variabel yang sama di [theme.css](theme.md). Karena komponen menggunakan `var()`, warna dapat berganti tanpa mengubah setiap selector.

