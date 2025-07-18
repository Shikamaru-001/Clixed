CREATE TABLE IF NOT EXISTS users (
    id uuid DEFAULT gen_random_uuid(),
    username VARCHAR(255) NOT NULL UNIQUE
  );

INSERT INTO users (username) VALUES
  ('thefirst'),
  ('thesecond')
ON CONFLICT (username) DO NOTHING;
