# Catatan Tahap 08 — JSON array

Tahap ini mempertahankan seluruh lapisan CRUD dan presenter Rupiah dari tahap
07. Perubahan hanya pada bentuk response untuk endpoint daftar laptop.

Endpoint `GET /api/laptops` mengembalikan object dengan `status` dan `data`.
Isi `data` berupa array dengan urutan `[id, nama, merek, harga, stok]`.
Endpoint detail dan operasi CRUD tetap memakai presenter laptop.
