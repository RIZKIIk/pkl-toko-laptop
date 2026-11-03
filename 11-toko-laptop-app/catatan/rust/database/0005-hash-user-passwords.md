# `database/migrations/0005_hash_user_passwords.sql`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md)

[Buka file migration SQL](../../../database/migrations/0005_hash_user_passwords.sql)

Migration ini menyiapkan kolom `password_hash` baru dan mengganti nama kolom password teks biasa menjadi `legacy_password` sementara. Setelah migration dijalankan, proses startup meng-hash setiap password lama dengan Argon2id dan salt acak, mengosongkan nilai lama, memastikan hash tidak boleh `NULL`, lalu menghapus kolom legacy.

Password lama yang masih tersimpan sebagai nilai nonkosong tetap bisa digunakan untuk login setelah di-hash. Migration 0003 pernah mereset password akun yang lebih tua; akun admin disinkronkan kembali dari environment, sedangkan akun lama lain yang password-nya kosong perlu diatur ulang.

Upgrade dilindungi PostgreSQL advisory lock agar dua instance aplikasi tidak memproses kolom legacy secara bersamaan. Jika proses terhenti di tengah, hash yang sudah tersimpan tetap dipertahankan dan startup berikutnya dapat melanjutkan.

Migration lama tidak diubah agar checksum SQLx tetap cocok dengan database yang sudah ada. Jangan mengedit migration yang sudah pernah dijalankan; buat file baru untuk perubahan selanjutnya.
