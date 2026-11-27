# `sidebar.js` — Menu Desktop dan Drawer Ponsel

> **Navigasi:** [Indeks JavaScript](README.md) · [Source](../../js/sidebar.js) · [Markup](dashboard-view.md) · [CSS sidebar](../css/dashboard.md) · [Aturan mobile](../css/responsive.md)

Modul ini mengelola menu aktif, sidebar ringkas desktop, dan drawer navigasi ponsel.

## Menentukan ukuran layout

`MOBILE_LAYOUT` berisi media query `(max-width: 620px)`. `isMobileLayout()` menanyakan nilainya ke `window.matchMedia()`. Angka ini harus cocok dengan breakpoint pada [responsive.css](../css/responsive.md).

`updateToggleLabel()` memperbarui `aria-expanded`, `aria-label`, dan `title` pada kontrol sidebar. Ia juga menyinkronkan `inert` dan `aria-hidden` sidebar, serta menonaktifkan area utama saat drawer terbuka. Jika ukuran window berubah dari drawer terbuka ke desktop, class drawer dan scroll lock dibersihkan.

## Buka dan tutup drawer

Di ponsel, tombol teks **Menu** memanggil `toggleSidebar()`, yang menambah/menghapus class `open` pada sidebar dan backdrop, mengunci scroll lewat `body.sidebar-open`, lalu memindahkan fokus ke tombol tutup atau tombol pembuka. Di desktop, klik logo atau area sidebar yang bukan tautan/tombol mengubah `state.sidebarCollapsed` dan class `sidebar-collapsed`.

Drawer dapat ditutup lewat tombol X, backdrop, tombol Escape, atau memilih tautan navigasi. Setelah ditutup, fokus dikembalikan ke tombol menu. Event delegation pada `document` menjaga listener tetap bekerja setelah dashboard dirender ulang.

## Keyboard dan aksesibilitas

- Saat drawer terbuka, tombol Escape memanggil `closeMobileSidebar()`.
- Listener Tab membatasi perpindahan fokus pada tautan dan tombol di drawer. `event.shiftKey` memeriksa arah Shift+Tab; jika fokus melewati ujung daftar, fokus dipindahkan ke ujung satunya.
- `inert` membuat elemen dan turunan tidak bisa difokuskan atau diaktifkan. `aria-hidden` menyampaikan state tersembunyi kepada teknologi bantu.
- `aria-current="page"` menandai tautan aktif.

## Perubahan ukuran window

Listener `resize` memanggil `updateToggleLabel()` agar tampilan, state, label aksesibel, dan status interaksi tetap sesuai saat ponsel diputar atau ukuran browser berubah.

