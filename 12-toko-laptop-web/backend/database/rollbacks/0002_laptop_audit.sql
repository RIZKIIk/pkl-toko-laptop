-- Guarded rollback for 0002_laptop_audit.sql.
-- Refuse to remove actor data or detach audit rows for laptops deleted after migration.
BEGIN;

DO $rollback$
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.laptop_logs
        WHERE laptop_id IS NULL OR user_id IS NOT NULL
    ) THEN
        RAISE EXCEPTION
            'Rollback refused: audit rows contain preserved deleted-laptop references or actor IDs';
    END IF;
END;
$rollback$;

DROP TRIGGER IF EXISTS laptops_capture_audit_log ON public.laptops;
DROP FUNCTION IF EXISTS public.capture_laptop_log();

ALTER TABLE public.laptop_logs DROP CONSTRAINT IF EXISTS laptop_logs_user_id_fkey;
ALTER TABLE public.laptop_logs DROP CONSTRAINT IF EXISTS laptop_logs_laptop_id_fkey;
ALTER TABLE public.laptop_logs DROP COLUMN IF EXISTS user_id;
ALTER TABLE public.laptop_logs ALTER COLUMN laptop_id SET NOT NULL;
ALTER TABLE public.laptop_logs
    ADD CONSTRAINT fk_laptop_logs_laptops
    FOREIGN KEY (laptop_id) REFERENCES public.laptops(id);

DROP INDEX IF EXISTS public.laptop_logs_user_id_idx;
DROP INDEX IF EXISTS public.laptop_logs_laptop_id_idx;

DELETE FROM public._sqlx_migrations WHERE version = 2;

COMMIT;
