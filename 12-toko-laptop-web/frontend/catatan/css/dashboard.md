# `dashboard.css` — Dashboard dan Sidebar

> **Navigasi:** [Indeks CSS](README.md) · [Source](../../css/dashboard.css) · [Markup dashboard](../js/dashboard-view.md) · [Interaksi sidebar](../js/sidebar.md)

Bagian awal file membentuk grid dashboard, sidebar gelap, navigasi, header, area konten, ringkasan, dan tabel inventaris. Di bagian akhir terdapat polesan tambahan untuk sidebar dan fokus interaksi.

## Tata letak dashboard

- `.app-layout` memakai `display: grid` dengan dua kolom: sidebar selebar 246px dan konten yang mengisi sisa ruang.
- `.app-layout.sidebar-collapsed` mengubah kolom pertama menjadi 76px. Di desktop, klik logo atau area kosong sidebar untuk mengecilkan/membesarkannya; navigasi dan tombol tetap menjalankan aksinya masing-masing.
- `.sidebar` adalah kolom navigasi dengan Flexbox arah kolom. `margin-top: auto` pada `.sidebar-bottom` mendorong bantuan, profil, dan logout ke bawah.
- `.main-area` memakai `min-width: 0` agar tabel lebar dapat digulir tanpa merusak grid.
- `.content` membatasi lebar maksimum dan memakai `clamp()` untuk padding yang menyesuaikan layar.

## Keadaan menu

- `.nav-link:hover` menampilkan keadaan hover.
- `.nav-link.active` menandai bagian yang aktif; selector ini lebih spesifik daripada `.nav-link` biasa.
- `.sidebar-collapsed .nav-label` disembunyikan ketika sidebar desktop diciutkan; ikon tetap terlihat dan `title`/`aria-label` memberi nama.
- `.sidebar-header` menempatkan brand dan tombol tutup drawer pada satu baris.
- `.sidebar-brand-toggle` membuat logo menjadi kontrol keyboard yang bisa mengecilkan atau membesarkan sidebar desktop.
- `.sidebar-dismiss` disembunyikan pada desktop dan ditampilkan melalui aturan responsive ponsel.
- `.sidebar-shade.open` menampilkan lapisan gelap di belakang drawer ponsel.
- `body.sidebar-open` mengunci scroll halaman di belakang drawer.

## Komponen dashboard

File ini juga menata breadcrumb, kartu statistik, panel inventaris, badge role, chip stok, tombol aksi, empty state, dan error state. Warna utama memakai custom properties seperti `var(--paper)`, `var(--line)`, dan `var(--shadow)` supaya perubahan tema terhubung dengan `theme.css`.

## Mengubah navigasi dengan aman

Sidebar memiliki markup di [dashboard-view.js](../js/dashboard-view.md), respons buka/tutup di [sidebar.js](../js/sidebar.md), dan breakpoint ponsel di [responsive.css](responsive.md). Saat mengganti nama, ikon, atau class, sesuaikan ketiganya agar label, fokus keyboard, dan tampilan tetap sejalan.

