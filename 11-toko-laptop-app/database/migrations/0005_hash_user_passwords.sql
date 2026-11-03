ALTER TABLE public.users
    RENAME COLUMN password TO legacy_password;

ALTER TABLE public.users
    ALTER COLUMN legacy_password DROP NOT NULL,
    ADD COLUMN password_hash TEXT;
