# `src/auth/jwt.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

[Buka kode sumber `jwt.rs`](../../../src/auth/jwt.rs)

## Tanggung jawab

Modul ini menerbitkan dan memverifikasi access token JWT dengan algoritma HMAC-SHA256 (HS256). Login memasukkan username sebagai subject token dan menetapkan expiry satu jam. Secret dibaca dari `JWT_SECRET`, yang harus berisi minimal 32 byte.

## Alur token

1. `JwtService` menyimpan secret di `Arc<str>` sehingga service murah untuk di-clone ke middleware dan handler.
2. `issue_token` membentuk claims `sub` dan `exp`, lalu menandatanganinya dengan HS256.
3. `verify_token` menetapkan algoritma HS256 secara eksplisit, memeriksa signature dan expiry, lalu mengembalikan username.
4. `bearer_token` hanya menerima header `Authorization` dengan skema `Bearer`; `username_from_headers` mengembalikan identitas hanya jika token valid.

Client mengirim token pada header `Authorization: Bearer <access_token>`. Token yang hilang, berubah, ditandatangani secret lain, atau kedaluwarsa tidak memberi akses.

## Batas logout

JWT bersifat stateless dan tidak dicabut oleh endpoint logout. Logout menghapus session cookie, sementara JWT yang sudah diterbitkan dapat dipakai sampai masa berlaku satu jam berakhir. Client harus menghapus token setelah logout.

## Pengujian

Unit test memastikan token dapat diterbitkan dan diverifikasi, serta menolak token yang diubah atau memakai secret berbeda.
