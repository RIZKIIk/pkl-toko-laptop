-- Restore the obsolete column as zero-valued data if this migration is rolled back.
BEGIN;

ALTER TABLE public.users
    ADD COLUMN IF NOT EXISTS token_version BIGINT NOT NULL DEFAULT 0
        CHECK (token_version >= 0);

DELETE FROM public._sqlx_migrations WHERE version = 3;

COMMIT;
