-- Migration latihan session: ganti hash lama dengan kolom password sederhana.
-- Password akun yang ada di-reset; akun seed dari env disinkronkan saat startup.
ALTER TABLE public.users
    DROP COLUMN password_hash,
    ADD COLUMN password TEXT NOT NULL DEFAULT '';

ALTER TABLE public.users
    ALTER COLUMN password DROP DEFAULT;
