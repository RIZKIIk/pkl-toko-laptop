# `laptop-actions.js` — Detail dan Operasi Produk

> **Navigasi:** [Indeks JavaScript](README.md) · [Source](../../js/laptop-actions.js) · [API dashboard](dashboard.md) · [Helper](shared.md)

Modul ini menampilkan dialog dan mengirim request API untuk detail, tambah, ubah, hapus, serta pembelian laptop.

## Dialog dan form

`showDialog(title, subtitle, body, onReady)` membentuk isi elemen `<dialog>`, membuka dialog dengan `showModal()`, menjalankan callback setelah markup terpasang, dan memfokuskan field pertama yang cocok. `wasOpen` menjaga dialog tidak dibuka ulang jika kontennya diganti saat masih terbuka.

`laptopForm(laptop = {})` membuat form yang bisa digunakan untuk tambah atau edit. Default object `{}` membuat argumen opsional aman dibaca. Operator `||` memberi string kosong sebagai fallback; `??` memberi fallback hanya jika harga belum ada. Atribut `required`, `min`, `max`, dan `step` membantu input angka.

`parseRupiah()` mengubah format harga seperti `Rp 12.500.000` menjadi angka dengan menyisakan digit saja.

## Tambah dan edit

`bindLaptopForm()` mencegah submit default, membaca nilai `form.elements`, mengubah harga/stok menjadi `Number`, dan mengirim JSON. Ternary memilih URL dan method: produk baru memakai `POST /api/laptops`; edit memakai `PATCH /api/laptops/{id}`. Setelah sukses, dialog ditutup, toast ditampilkan, lalu dashboard dimuat ulang.

`openCreate()` memeriksa role admin sebelum menampilkan form. `openEdit(id)` meminta detail terbaru, menormalkan response, lalu membuka form dengan nilai awal.

## Detail dan pembelian

`openDetail(id)` memakai `encodeURIComponent()` agar ID aman disisipkan ke URL, meminta detail terbaru, lalu menghitung apakah role boleh membeli, edit, atau menghapus. Kondisi role hanya mengatur tombol UI; backend tetap yang berwenang.

Jika pembelian tersedia, form jumlah dikirim dengan `POST /api/laptops/{id}/buy`. Jumlah diubah menjadi angka; hasil dipakai untuk pesan toast, lalu daftar dashboard diperbarui.

## Hapus produk

`deleteLaptop(id)` mencari produk di state untuk membuat kalimat konfirmasi. Optional chaining (`item?.nama`) aman saat item tidak ditemukan. Jika `window.confirm()` membatalkan aksi, fungsi langsung `return`. Jika disetujui, request `DELETE` dikirim dan dashboard di-refresh setelah sukses.

Semua operasi request memakai `try/catch` dan `reportError()` agar error API tampil sebagai pesan yang bisa dipahami pengguna.

