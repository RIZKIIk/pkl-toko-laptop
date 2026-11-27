-- Index the optional laptop foreign key for lookups and referential actions.
CREATE INDEX IF NOT EXISTS purchases_laptop_id_idx
    ON public.purchases (laptop_id);

-- Guard receipt totals even when data is written outside the application.
DO $migration$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conrelid = 'public.purchases'::regclass
          AND conname = 'purchases_total_matches_check'
    ) THEN
        ALTER TABLE public.purchases
            ADD CONSTRAINT purchases_total_matches_check
            CHECK (total_price = unit_price * quantity) NOT VALID;
    END IF;
END;
$migration$;

ALTER TABLE public.purchases
    VALIDATE CONSTRAINT purchases_total_matches_check;
