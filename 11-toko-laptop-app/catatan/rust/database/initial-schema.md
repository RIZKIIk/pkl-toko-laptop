# Skema database final

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

## Tujuan

Skema final aplikasi terdiri dari tabel `laptops` dan `users`. File migration SQL mempertahankan urutan historis `0001`–`0008` agar database yang sudah berisi data dapat menjalankan migration lanjutan dengan checksum SQLx yang cocok. Jangan menghapus atau mengubah migration yang pernah diterapkan, dan jangan squash/menyatukan migration lama menjadi satu file baru: database yang telah mencatat migration lama perlu melihat nomor serta checksum historis yang sama. Perubahan berikutnya harus ditambahkan sebagai migration bernomor baru.

SQLx mencatat migration yang sudah dijalankan dalam tabel riwayat migration. Saat startup, macro `sqlx::migrate!` menyertakan berkas SQL dalam build dan `.run(&pool)` menjalankan hanya migration yang belum tercatat. Urutan nomor membuat perubahan skema dapat diterapkan bertahap pada database baru maupun database lama.

## Tabel `laptops`

| Kolom | Tipe | Kegunaan |
| --- | --- | --- |
| `id` | `BIGINT` identity | Primary key. |
| `nama`, `merek` | `VARCHAR` | Nama dan merek wajib; nilai kosong ditolak. |
| `harga` | `BIGINT` | Harga rupiah, tidak boleh negatif. |
| `stok` | `INTEGER` | Jumlah stok, tidak boleh negatif. |
| `created_at`, `updated_at` | `TIMESTAMPTZ` | Waktu pembuatan dan perubahan. |

Validasi di database menjadi lapisan perlindungan tambahan bagi [validasi Rust](../validation.md). Query PATCH memperbarui `updated_at`.

## Tabel `users`

| Kolom | Tipe | Kegunaan |
| --- | --- | --- |
| `id` | `BIGINT` identity | Primary key. |
| `username` | `VARCHAR(100)` | Username unik dan tidak boleh kosong. |
| `password_hash` | `TEXT` | Hash password Argon2id. |
| `role` | `VARCHAR(20)` | Role `admin`, `buyer`, atau `restocker`; default `buyer`. |
| `token_version` | `BIGINT` | Versi JWT; logout menaikkannya untuk mencabut semua JWT lama pengguna. |
| `created_at` | `TIMESTAMPTZ` | Waktu akun dibuat. |

Migration `0008` menambahkan `token_version` (dengan nilai awal yang sesuai untuk akun yang ada) dan menghapus tabel `revoked_jwts`; database lama akan menerapkannya otomatis saat `cargo run`. Akun lama tetap dipertahankan, termasuk hash password-nya. Versi disimpan per pengguna; logout menaikkannya, lalu middleware membandingkan versi pada JWT dengan database. Ini mencabut semua JWT lama pengguna sekaligus, bukan session cookie lain.

## Istilah SQL dalam skema

- **Primary key** (`id`) mengidentifikasi satu baris secara unik. `BIGINT ... IDENTITY` meminta PostgreSQL menghasilkan angka ID.
- **Unique** pada username mencegah dua akun memakai nama yang sama; **NOT NULL** mewajibkan kolom memiliki nilai.
- `TIMESTAMPTZ` menyimpan suatu instant waktu dengan dukungan zona waktu; PostgreSQL menormalisasinya dan tidak menyimpan nama zona asli yang diberikan client. `DEFAULT` mengisi nilai bila query tidak menyediakannya.
- Constraint `CHECK` dapat membatasi nilai (misalnya harga/stok tidak negatif); validasi Rust memberi pesan ramah lebih awal, sedangkan constraint database melindungi data dari semua jalur penulisan.
- Tipe `BIGINT`/`INTEGER` membatasi rentang bilangan; nilai Rupiah disimpan sebagai bilangan bulat, bukan pecahan floating-point.
