# `responsive.css` — Tablet dan Ponsel

> **Navigasi:** [Indeks CSS](README.md) · [Source](../../css/responsive.css) · [Perilaku drawer](../js/sidebar.md)

File ini berisi aturan media query. CSS dasar dipakai lebih dulu, lalu aturan responsive menyesuaikan layout ketika layar menyempit.

## Breakpoint yang ada

- `@media (max-width: 820px)` menyesuaikan dashboard dan konten untuk tablet atau jendela sempit.
- `@media (max-width: 620px)` menyusun ulang form akun dan mengubah dashboard menjadi satu kolom.
- Pada ponsel, `.sidebar` menjadi `position: fixed` dan bergeser keluar layar menggunakan `transform: translateX(-110%)`. Class `.open` menggesernya masuk.
- `.sidebar-shade` menutupi konten saat drawer dibuka; `.sidebar-shade.open` membuatnya terlihat.
- Aturan ponsel yang lebih baru menyempurnakan lebar drawer, scroll internal, ukuran tombol menu/tutup, jarak navigasi, dan tampilan ketika state desktop sebelumnya tersimpan.

## Mengapa ada dua blok media query 620px?

Keduanya memiliki kondisi yang sama. CSS membaca file dari atas ke bawah; aturan di blok yang lebih bawah dapat menimpa aturan sebelumnya jika properti dan specificity setara. Blok kedua dipakai untuk tambahan polish sidebar tanpa mengubah kelompok layout mobile yang sudah ada. Saat menambah aturan, pastikan perubahan memang menang pada urutan cascade.

## Mengubah breakpoint

Breakpoint CSS harus sejalan dengan `MOBILE_LAYOUT` di [sidebar.js](../js/sidebar.md), yang menentukan kapan tombol menjadi drawer ponsel. Jika angka hanya diubah pada satu sisi, tampilan dan perilaku bisa berbeda di sekitar batas tersebut.

