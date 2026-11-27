# `user-actions.js` — Pembuatan Akun oleh Admin

> **Navigasi:** [Indeks JavaScript](README.md) · [Source](../../js/user-actions.js) · [Dashboard](dashboard-view.md) · [Endpoint akun](../../../backend/endpoin.md)

Modul ini membuka form admin untuk membuat akun pengguna. Form mengambil role dari `GET /api/admin/roles`, lalu mengirim username, password, dan role pilihan ke `POST /api/admin/users`.

## Memuat pilihan role

`openCreateUser()` memeriksa role pengguna di `state.user` sebelum membuka form. Backend tetap menjadi pemeriksa akses utama. Request daftar role ditangani dengan `try/catch`; jika gagal atau daftar kosong, pesan toast ditampilkan.

Opsi `<select>` dibuat dari array `roles` memakai `map(...).join("")`. `esc()` meloloskan value dan label sebelum dimasukkan ke template HTML. Deskripsi role berubah saat pilihan berganti, memakai event `change` dan `Array.find()`.

Frontend tidak menulis daftar role tetap di JavaScript. Setelah role baru ditambahkan ke model role, izin akses, dan constraint database backend, admin dapat memasukkannya ke daftar role yang boleh dibuat; dropdown akan menerima pilihan itu dari response API.

## Mengirim form

Handler `submit` mencegah reload halaman, menonaktifkan tombol selama request, memangkas spasi username, lalu membuat body JSON dengan `JSON.stringify()`. Jika berhasil, dialog ditutup dan toast mengonfirmasi akun. Jika gagal, pesan API ditampilkan di dalam dialog dan tombol diaktifkan lagi.

Password hanya dikirim ke backend melalui HTTPS di deployment sungguhan. Backend melakukan hashing; frontend tidak menyimpan password baru ke storage browser.

## Relasi dengan backend

Endpoint `GET /api/admin/roles` dan `POST /api/admin/users` dilindungi autentikasi admin. Registrasi publik tetap memakai endpoint terpisah yang selalu menetapkan role buyer. Role baru juga memerlukan definisi izin di backend; menambahkannya ke dropdown saja tidak memberi akses API.

