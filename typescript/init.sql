-- Initialize chaindexing database
-- This file runs when the database is first created

-- Create the main database (already done by POSTGRES_DB env var)
-- CREATE DATABASE chaindexing;

-- Set up database for chaindexing
\c chaindexing;

-- Grant necessary permissions
GRANT ALL PRIVILEGES ON DATABASE chaindexing TO chaindexing_user;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO chaindexing_user;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO chaindexing_user;

-- Create extension for UUID generation if needed
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- The application will create the tables via migrations
-- This file just sets up the initial database structure 