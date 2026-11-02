# Catatan Tahap 09 — Login dan logout

Tahap ini membawa modul database dan fitur laptop sebelumnya, lalu
menambahkan autentikasi berbasis cookie session.

## Modul tambahan

- `handlers/auth.rs`: login, membaca session aktif, dan logout.
- `repository.rs`: query laptop serta verifikasi akun dari `public.users`.
- `routes.rs`: mendaftarkan endpoint autentikasi dan laptop.
- `presenters.rs`: mempertahankan format response laptop dan Rupiah.

Akun harus sudah tersedia di tabel `public.users`. Tahap ini tidak membuat
tabel, menjalankan migration, ataupun melakukan seed akun.
