-- Create the admin user if it doesn't exist
DO
$$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'address_admin') THEN
      CREATE USER address_admin WITH PASSWORD 'admin12345';
   END IF;
END
$$;

-- Drop the database if it exists and create a new one owned by address_admin
-- This ensures a clean state for development.
DROP DATABASE IF EXISTS address_book;
CREATE DATABASE address_book OWNER address_admin;
