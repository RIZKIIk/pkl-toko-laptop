# Catatan Tahap 10 — Middleware session

Source modular tahap 09 tetap disertakan langsung dalam folder ini. Perubahan
tahap 10 menambahkan `middleware.rs` dan mengelompokkan route menjadi route
publik serta route yang membutuhkan session.

- `GET /health` dan `POST /api/auth/login` dapat diakses tanpa login.
- Session, logout, dan seluruh endpoint laptop memerlukan cookie session.
- Koneksi memakai tabel `public.users` dan `public.laptops` dari skema tahap
  11; tidak ada migration atau perubahan skema di tahap ini.
