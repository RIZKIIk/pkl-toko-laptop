# `database/migrations/0004_add_auth_version.sql`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md)

[Buka file migration SQL](../../../database/migrations/0004_add_auth_version.sql)

Migration ini menambahkan `auth_version` ke `public.users` sebagai bilangan `BIGINT` dengan nilai awal `0`. Migration berikutnya, [0007](0007-revoke-jwts.md), menghapus kolom ini dan memakai denylist per-JWT.

Migration ini merupakan bagian dari riwayat skema lama. Pada skema terbaru, logout mencabut JWT yang dipakai melalui `jti` dan menghapus session aktif.

Jangan mengubah migration setelah diterapkan; tambahkan file migration baru untuk perubahan skema berikutnya.
