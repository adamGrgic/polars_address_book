-- Create the user if it doesn't exist.
DO
$$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'address_admin') THEN
      CREATE USER address_admin WITH PASSWORD 'admin12345';
   END IF;
END
$$;

-- Grant privileges. Note that some of these might require superuser access
-- and might be better handled outside of a migration script.
GRANT ALL PRIVILEGES ON DATABASE address_book TO address_admin;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO address_admin;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT USAGE, SELECT ON SEQUENCES TO address_admin;

-- Grant usage on the public schema.
GRANT USAGE ON SCHEMA public TO address_admin;
-- Grant all privileges on all tables in the public schema to the new user
GRANT ALL ON ALL TABLES IN SCHEMA public TO address_admin;
-- Grant all privileges on all sequences in the public schema to the new user
GRANT ALL ON ALL SEQUENCES IN SCHEMA public TO address_admin;


-- Add up migration script here
CREATE TABLE contacts (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255),
    email VARCHAR(255) UNIQUE,
    phone VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);