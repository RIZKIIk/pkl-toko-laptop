# `dashboard-view.js` — Markup dan Daftar Laptop

> **Navigasi:** [Indeks JavaScript](README.md) · [Source](../../js/dashboard-view.js) · [Data dashboard](dashboard.md) · [Aksi produk](laptop-actions.md) · [Sidebar](sidebar.md)

Modul ini mengubah state dashboard menjadi HTML: ringkasan stok, filter, tabel, keadaan kosong, dan sidebar.

## Normalisasi data

`normalizeLaptop(row)` memeriksa `Array.isArray(row)`. Jika API mengirim array, object baru dibuat dengan nama property yang jelas; jika sudah object, row dipakai langsung. Operator ternary memilih salah satu hasil.

## Pencarian, filter, dan urutan

`inventoryMarkup(laptops)`:

1. Mengubah pencarian menjadi huruf kecil dengan locale Indonesia.
2. Memakai `filter()` untuk mencocokkan nama, merek, dan harga.
3. Memfilter stok berdasarkan pilihan (`available`, `low`, atau `out`).
4. Memakai `sort()` dengan comparator untuk nama atau harga.
5. Membuat empty state atau tabel sesuai jumlah produk yang terlihat.

`trim()` membuang spasi tepi. `Number()` mengubah teks angka menjadi number. `String(...).replace(/[^\d]/g, "")` membuang karakter harga yang bukan digit sebelum perbandingan angka.

## Template HTML dan keamanan

Fungsi `html` dari `shared.js` membantu menulis template multiline. Ekspresi `${...}` menyisipkan nilai. Data dari API seperti nama, merek, ID, dan harga dilewatkan ke `esc()` sebelum disisipkan ke `innerHTML`. Untuk markup bersyarat dipakai ternary; untuk daftar baris digunakan `map(...).join("")`.

## Dashboard dan statistik

`renderDashboard()` membaca `state.user` dan `state.laptops`. `reduce()` menjumlahkan stok, `filter()` menghitung produk stok rendah, `Intl.DateTimeFormat("id-ID")` membuat tanggal lokal, dan `charAt(0)` mengambil inisial akun.

Tombol dibuat dengan `data-action` supaya dapat ditangani listener global di `app.js`. Admin mendapat tombol **Tambah akun**, sedangkan role lain tidak melihatnya. Form dan pilihan role ditangani oleh [user-actions.js](user-actions.md); backend tetap memeriksa akses admin.

## Sidebar

Markup `<aside id="sidebar">` berisi brand, nav, bantuan, profil, dan logout. Di desktop, logo menjadi kontrol ciutkan/besarkan; klik area kosong sidebar juga mengubah lebarnya. Di ponsel, tombol teks **Menu** membuka drawer. Kontrol memakai `aria-controls`, `aria-expanded`, dan label yang berubah. Saat drawer tertutup, `inert` dan `aria-hidden` mencegah interaksi keyboard/pembaca layar. Logika lengkapnya ada di [sidebar.js](sidebar.md), gaya visualnya di [dashboard.css](../css/dashboard.md).

