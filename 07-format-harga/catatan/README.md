# Catatan Tahap 07 — Format harga

Semua modul koneksi, server, CRUD, model, repository, handler, error, dan
route dari tahap 06 tetap ada.

## Penambahan tahap ini

`presenters.rs` mengubah entity database menjadi bentuk response API.
Harga tetap disimpan sebagai angka rupiah di PostgreSQL, tetapi dikirim ke
client sebagai teks seperti `Rp15.000.000,00`. Test presenter memeriksa
pemisah ribuan dan nilai batas.
