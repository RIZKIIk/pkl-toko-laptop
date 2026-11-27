# `theme.js` — Tombol Terang/Gelap

> **Navigasi:** [Indeks JavaScript](README.md) · [Source](../../js/theme.js) · [Inisialisasi awal](theme-init.md) · [CSS tema](../css/theme.md)

Modul ini membuat markup tombol tema dan mengubah preferensi tema saat tombol diklik.

## Membaca dan mengganti tema

- `temaAktif()` membaca `document.documentElement.dataset.theme` dan memberi nilai default `light`.
- `themeToggleMarkup()` membentuk tombol memakai template literal, ikon, label, `aria-pressed`, dan `aria-label` sesuai state saat ini.
- `toggleTheme()` menghitung nilai kebalikan melalui ternary, mengubah `dataset.theme`, dan memperbarui warna meta browser.
- `localStorage.setItem()` menyimpan preferensi agar pilihan bertahan lintas tab/sesi. `try/catch` menjaga fungsi visual tetap berjalan bila storage tidak tersedia.
- `querySelectorAll()` mengambil semua tombol tema yang sedang tampil; `forEach()` memperbarui atribut dan teks masing-masing.

`aria-pressed` menyampaikan apakah tombol toggle sedang aktif. Label aksesibel menyebut tindakan berikutnya, seperti “Ubah ke mode gelap”. CSS tema lalu membaca `data-theme` pada elemen akar.

