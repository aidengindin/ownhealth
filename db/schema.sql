CREATE TABLE IF NOT EXISTS user(
  id BIGSERIAL PRIMARY KEY,
  email TEXT UNIQUE NOT NULL,
  password_hash TEXT NOT NULL,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  created_ts TIMESTAMPTZ,
  updated_ts TIMESTAMPTZ
);

CREATE OR REPLACE FUNCTION create_metric_table(
  table_name TEXT,
  value_type TEXT,
  additional_columns TEXT DEFAULT ''
) RETURNS void AS $$
BEGIN
  EXECUTE format('
    CREATE TABLE IF NOT EXISTS %I (
      id BIGSERIAL PRIMARY KEY,
      user_id BIGINT NOT NULL,
      timestamp TIMESTAMPTZ NOT NULL,
      value %s NOT NULL%s,
      created_ts TIMESTAMPTZ,
      updated_ts TIMESTAMPTZ,
      CONSTRAINT %I FOREIGN KEY (user_id) REFERENCES user(id)
      ON DELETE CASCADE
    );
    CREATE INDEX %I ON %I (timestamp);
    CREATE INDEX %I on %I (user_id);
    CREATE INDEX %I ON %I (user_id, timestamp DESC);
    SELECT create_hypertable(%L, ''timestamp'');
  ',
  table_name,
  value_type,
  additional_columns,
  table_name || '_user_fk',
  table_name || '_timestamp_idx',
  table_name,
  table_name || '_user_idx',
  table_name,
  table_name || '_user_time_idx',
  table_name,
  table_name);
END;
$$ LANGUAGE plpgsql;;

-- Create sleep_stage enum type for sleep stage values
DO $$ 
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'sleep_stage') THEN
    CREATE TYPE sleep_stage AS ENUM ('awake', 'light', 'deep', 'rem');
  END IF;
END $$;

-- Create tables for all data types defined in domain.rs
SELECT create_metric_table('heart_rate', 'INT');
SELECT create_metric_table('weight', 'DECIMAL(5,2)');
SELECT create_metric_table('hydration', 'DECIMAL(6,2)');
SELECT create_metric_table('vo2_max', 'DECIMAL(3,1)');
SELECT create_metric_table('sleep_duration', 'INT');
SELECT create_metric_table('sleep_stages', 'sleep_stage');

