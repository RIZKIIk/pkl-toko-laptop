# `register.js` — Pendaftaran Pembeli

> **Navigasi:** [Indeks JavaScript](README.md) · [Source](../../js/register.js) · [Helper API](shared.md) · [Login](login.md)

`renderRegister()` membentuk form pembuatan akun dan menangani submit tanpa memuat ulang dokumen.

Form publik ini hanya membuat akun pembeli; pengguna tidak dapat memilih role sendiri. Admin membuat akun dengan role lain melalui [user-actions.js](user-actions.md).

Panel promosi memakai SVG laptop lokal yang sama dengan halaman login. Ukuran layar sempit menyembunyikan panel tersebut dan menampilkan logo di atas form.

Kolom username memakai ikon pengguna dan kedua kolom password memakai ikon gembok. Ikon hanya dekorasi (`aria-hidden`); label yang terhubung ke input tetap menjadi nama aksesibelnya.

## Validasi browser

Handler submit mengambil username, password, dan konfirmasi dari `form.elements`. Username di-`trim()`, sementara password tidak diubah supaya spasi yang mungkin memang dipilih tidak dihapus.

Pemeriksaan pertama memastikan semua nilai terisi. Pemeriksaan kedua membandingkan password dan konfirmasi dengan `!==`. Jika berbeda, pesan ditampilkan dan fokus diarahkan ke input konfirmasi.

Form menggunakan `novalidate`, jadi pemeriksaan sederhana ditangani sendiri oleh JavaScript. Atribut `required` dan `maxlength` masih memberi informasi constraint pada input.

## Request dan hasil

Sebelum mengirim, tombol dinonaktifkan dan teksnya berubah. `api("/api/auth/register", ...)` mengirim `POST` JSON tanpa autentikasi (`auth: false`). Jika berhasil, browser berpindah ke `/login?registered=1`; `login.js` membaca query tersebut lalu menampilkan pesan berhasil. Jika gagal, `catch` menampilkan error dan mengembalikan tombol.

Validasi keamanan akun tetap harus dilakukan backend, meskipun frontend memberi umpan balik agar pengguna mudah memperbaiki input.

