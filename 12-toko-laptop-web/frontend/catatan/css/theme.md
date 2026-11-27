# `theme.css` — Mode Gelap

> **Navigasi:** [Indeks CSS](README.md) · [Source](../../css/theme.css) · [Perilaku tombol tema](../js/theme.md) · [Tema awal](../js/theme-init.md)

Aturan mode gelap diawali selector `:root[data-theme="dark"]`. JavaScript mengatur atribut `data-theme` pada elemen `<html>`; CSS kemudian mengganti custom properties dan beberapa komponen yang memerlukan warna khusus.

## Cara kerja selector

```css
:root[data-theme="dark"] {
  --ink: #e8eff0;
  --paper: #192427;
}
:root[data-theme="dark"] .stat-card {
  color: var(--ink);
  background: var(--paper);
}
```

Bagian pertama mengubah token warna. Bagian kedua memperbaiki selector tertentu yang sebelumnya memiliki warna latar langsung atau membutuhkan kontras khusus. Nilai `color-scheme: dark` juga memberi tahu browser agar kontrol native cocok dengan tema gelap.

## Saat menambah komponen

Mulai dengan memakai token dari `base.css` seperti `--ink`, `--muted`, `--paper`, `--line`, dan `--field-surface`. Tambahkan override di file ini hanya ketika warna komponen tidak mengikuti token atau perlu keadaan khusus. Hindari warna teks dan latar yang tidak punya pasangan agar kontras tetap terbaca.

