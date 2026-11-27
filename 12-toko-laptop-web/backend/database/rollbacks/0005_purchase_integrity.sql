-- Rollback for 0005_purchase_integrity.sql; preserves all purchase rows.
BEGIN;

ALTER TABLE public.purchases
    DROP CONSTRAINT IF EXISTS purchases_total_matches_check;
DROP INDEX IF EXISTS public.purchases_laptop_id_idx;
DELETE FROM public._sqlx_migrations WHERE version = 5;

COMMIT;
