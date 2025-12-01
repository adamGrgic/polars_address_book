CREATE DATABASE address_book;

DO
$$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'address_admin') THEN
      CREATE USER address_admin WITH PASSWORD 'admin12345';
      -- Optional: Grant privileges or set other attributes here
      -- GRANT CONNECT ON DATABASE your_database TO your_username;
      -- GRANT USAGE ON SCHEMA your_schema TO your_username;
   END IF;
END
$$;
