# 03 — Koneksi database

Tahap ini hanya memuat konfigurasi, membuka koneksi ke PostgreSQL, lalu
memverifikasi koneksi dengan `SELECT 1`. Program selesai setelah berhasil;
belum ada server HTTP atau fitur CRUD. Skema dan data tidak diubah.

Salin `.env.example` di folder tahap ini menjadi `.env`, lalu samakan nilai `DB_HOST`, `DB_PORT`,
`DB_NAME`, `DB_USER`, dan `DB_PASSWORD` dengan konfigurasi
[`11-toko-laptop-app-lengkap`](../11-toko-laptop-app-lengkap/README.md).
Jalankan:

```bash
cargo run --manifest-path 03-koneksi-database/Cargo.toml
```

Jika berhasil, terminal menampilkan pesan `Koneksi ke PostgreSQL berhasil.`
Jika konfigurasi atau database tidak tersedia, program menampilkan error
dan berhenti dengan status gagal. Tahap 04 melanjutkan proyek ini dengan
route, handler, dan server HTTP tanpa menghapus modul koneksinya.

Lihat [`catatan/README.md`](catatan/README.md) untuk penjelasan modul.
