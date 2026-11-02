-- Migration latihan session: ganti hash lama dengan kolom password sederhana.
-- Password akun yang ada di-reset; akun seed dari env disinkronkan saat startup.
ALTER TABLE public.users
    -- Hapus kolom hash awal; migrasi ini sengaja hanya untuk database latihan.
    DROP COLUMN password_hash,
    -- Password teks biasa digunakan oleh contoh auth lokal di aplikasi.
    ADD COLUMN password TEXT NOT NULL DEFAULT '';

-- Hapus default agar akun berikutnya harus mengirim password secara eksplisit.
ALTER TABLE public.users
    ALTER COLUMN password DROP DEFAULT;
