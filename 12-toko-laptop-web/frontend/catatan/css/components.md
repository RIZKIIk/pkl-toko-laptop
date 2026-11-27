# `components.css` — Komponen Bersama

> **Navigasi:** [Indeks CSS](README.md) · [Source](../../css/components.css)

File ini menata elemen yang dipakai lintas halaman: dialog native, tombol tutup, form di dialog, toast, indikator loading kecil, tombol sidebar, dan pengubah tema.

## Dialog

- `dialog` adalah elemen HTML bawaan; selector `.dialog[open]` berlaku saat atribut `open` ada.
- `::backdrop` menata lapisan belakang dialog modal.
- `position: sticky` pada `.dialog-head` menjaga judul dan tombol tutup terlihat saat isi dialog digulir.
- `.form-grid` memakai Grid; aturan responsive dapat mengubah jumlah kolom di layar kecil.

## Notifikasi dan kontrol

- `.toast.error` memberi variasi tampilan untuk error.
- `.theme-toggle` mengatur label dan ikon tombol tema; `.theme-icon-light` dan `.theme-icon-dark` dipilih melalui state tema pada `theme.css`.
- `.sidebar-toggle` adalah tombol teks **Menu** yang membuka/menutup drawer pada ponsel. Desktop memakai klik pada logo atau area kosong sidebar untuk mengubah lebarnya.
- `transition` mencantumkan properti dan durasi agar perubahan warna atau posisi berlangsung halus.

Markup dan event dialog berasal dari [laptop-actions.js](../js/laptop-actions.md); pesan toast dibuat oleh [shared.js](../js/shared.md). Hormati pilihan gerakan minim yang diatur di [base.css](base.md).

