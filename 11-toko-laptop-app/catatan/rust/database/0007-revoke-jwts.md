# `database/migrations/0007_revoke_jwts.sql`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md)

[Buka file migration SQL](../../../database/migrations/0007_revoke_jwts.sql)

Migration ini menghapus kolom `auth_version` dari `public.users` dan membuat tabel `revoked_jwts`. Tabel menyimpan `jti` token yang logout beserta waktu kedaluwarsanya.

Middleware memeriksa tabel ini pada setiap request JWT. Logout menambahkan `jti` token yang dipakai, lalu middleware menolak token tersebut sampai kedaluwarsa. Baris lama yang sudah kedaluwarsa dibersihkan saat logout berikutnya.
