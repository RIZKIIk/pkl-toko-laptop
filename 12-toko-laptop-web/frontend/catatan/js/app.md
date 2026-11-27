# `app.js` — Titik Masuk Aplikasi

> **Navigasi:** [Indeks JavaScript](README.md) · [Source](../../js/app.js) · [Helper bersama](shared.md)

`app.js` mengimpor modul fitur, memasang event listener umum, lalu memilih halaman berdasarkan `location.pathname`.

## Import dan fungsi logout

`import { ... } from "./shared.js"` mengambil export bernama. `logout()` adalah `async function`: tombol keluar dinonaktifkan, request `POST /api/auth/logout` ditunggu, token lokal dibersihkan, state pengguna dikosongkan, lalu browser berpindah ke `/login`. `try/catch` memungkinkan kegagalan request ditangani tanpa membuat proses lokal berhenti.

## Event delegation

Listener klik dipasang pada `document`, bukan setiap tombol. `event.target.closest("[data-action]")` mencari kontrol yang memiliki `data-action`. Pola ini tetap bekerja ketika JavaScript mengganti isi `#app`.

```js
const action = control.dataset.action;
if (action === "detail") openDetail(control.dataset.id);
```

Rangkaian kondisi menghubungkan `data-action` ke fungsi seperti buka/tutup dialog, ubah tema, tambah produk, refresh, dan logout. `instanceof Element` memastikan event berasal dari elemen HTML sebelum memanggil `closest()`.

Aksi `create-user` memanggil [user-actions.js](user-actions.md), yang mengambil pilihan role dari API admin dan membuat akun. Tombol ini hanya dirender untuk admin oleh `dashboard-view.js`.

## Input, perubahan filter, dan pemilihan halaman

- Event `input` memperbarui `state.search` setiap kali isi pencarian berubah.
- Event `change` membaca filter stok atau urutan produk.
- Setelah state diperbarui, `inventoryMarkup()` membuat ulang isi tabel.
- Tombol clear filter mengembalikan state dan nilai kontrol ke awal; `search?.focus()` memfokuskan pencarian bila elemen masih ada.
- `location.pathname` memilih dashboard, register, atau login. `location.assign()` membuat navigasi baru; `location.replace()` mengganti entry history.

Dialog disimpan di luar `#app`, jadi listener global pada `document` dapat menangani tombol di dalamnya walaupun konten dialog dibuat belakangan.

