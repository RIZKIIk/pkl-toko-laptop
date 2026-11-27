# `dashboard.js` — Session dan Pengambilan Data

> **Navigasi:** [Indeks JavaScript](README.md) · [Source](../../js/dashboard.js) · [Tampilan dashboard](dashboard-view.md) · [Helper API](shared.md)

File ini mengatur kapan dashboard boleh dibuka, memeriksa session, dan mengambil daftar laptop.

## Memuat data

`loadLaptops()` menunggu `api("/api/laptops")`, mengambil `result.data || []`, memetakan setiap row dengan `normalizeLaptop()`, lalu menaruh hasil ke `state.laptops`.

`map()` membuat array baru. Normalisasi menyamakan bentuk data dari API sebelum dipakai tampilan, sehingga view tidak perlu menebak-nebak struktur data.

## Memastikan session

`openDashboard()` memeriksa `location.pathname`. Untuk path yang bukan `/dashboard`, fungsi mencoba memeriksa session dan mengganti URL ke dashboard bila berhasil; bila gagal, login ditampilkan.

Pada path dashboard, fungsi meminta `GET /api/auth/session`. Hasil pengguna ditempatkan ke `state.user`, data laptop dimuat, dan `renderDashboard()` dipanggil. `catch` memeriksa jenis/status error: error selain `401` dilaporkan, lalu layar error dengan tombol retry dibuat untuk request yang gagal.

## Memuat ulang

`refreshDashboard()` mengambil daftar terbaru dan menggambar ulang dashboard. Bila API gagal, `reportError()` menunjukkan toast. Fungsi ini dipakai setelah pembelian atau perubahan data agar angka stok dan tabel mengikuti backend.

