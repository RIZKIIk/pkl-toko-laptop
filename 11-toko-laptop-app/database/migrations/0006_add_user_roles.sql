ALTER TABLE public.users
    ADD COLUMN role VARCHAR(20) NOT NULL DEFAULT 'buyer'
        CHECK (role IN ('admin', 'buyer', 'restocker'));
