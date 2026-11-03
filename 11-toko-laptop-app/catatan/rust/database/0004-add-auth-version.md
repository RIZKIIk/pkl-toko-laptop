# `database/migrations/0004_add_auth_version.sql`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md)

[Buka file migration SQL](../../../database/migrations/0004_add_auth_version.sql)

Migration ini menambahkan `auth_version` ke `public.users` sebagai bilangan `BIGINT` dengan nilai awal `0`. Setiap JWT dan session menyimpan versi yang berlaku ketika login.

Middleware membandingkan versi pada kredensial dengan nilai terbaru di database. Endpoint logout menaikkan `auth_version`, sehingga semua JWT dan session lama milik pengguna langsung tidak berlaku dan pengguna perlu login kembali.

Jangan mengubah migration setelah diterapkan; tambahkan file migration baru untuk perubahan skema berikutnya.
