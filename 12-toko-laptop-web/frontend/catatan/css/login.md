# `login.css` — Halaman Akun

> **Navigasi:** [Indeks CSS](README.md) · [Source](../../css/login.css) · [Markup login](../js/login.md) · [Markup daftar](../js/register.md)

File ini menata halaman masuk dan daftar, termasuk panel promosi, logo, form, input password, tombol, serta pesan error.

Versi saat ini memakai panel kiri bergradasi dengan ilustrasi laptop SVG lokal. Panel form memakai latar lembut, input berikon, state fokus yang jelas, dan tombol masuk beraksen emas. Aturan dasar desain berada di file ini; breakpoint ponsel/tablet ada di [responsive.css](responsive.md), sementara variasi mode gelap ada di [theme.css](theme.md).

## Pola CSS yang dipakai

- `.login-shell` memakai CSS Grid untuk membagi panel promosi dan panel form.
- `min()`, `max()`, dan `clamp()` memberi ukuran fleksibel yang tetap punya batas. Contoh umum: `width: min(450px, 100%)`.
- `::before` dan `::after` menambahkan ornamen dekoratif tanpa menambah elemen HTML.
- `.brand-mark svg` menarget SVG yang berada dalam ikon brand.
- `:focus` menata input saat sedang dipilih; selector ini berbeda dari `:focus-visible` umum di `base.css`.
- `.login-error.visible` menampilkan pesan hanya saat JavaScript memberi class `visible`; `.login-error.success` memberi gaya untuk pemberitahuan pendaftaran berhasil.
- `.login-submit:disabled` memberi tampilan saat form sedang mengirim request dan tombol tidak bisa ditekan lagi.

## Mengubah desain form

Markup berada di `login.js` dan `register.js`; keduanya menggunakan class seperti `.field`, `.input-wrap`, `.primary-button`, dan `.login-form-side`. Jika menambah class baru di markup, buat selector terkait di sini atau di `components.css`. Periksa [responsive.css](responsive.md) untuk tampilan ponsel dan [theme.css](theme.md) untuk mode gelap.

