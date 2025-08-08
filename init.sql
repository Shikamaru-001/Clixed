CREATE TABLE IF NOT EXISTS users (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE
  );

CREATE TABLE IF NOT EXISTS images (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  title VARCHAR(50) NOT NULL,
  description text
  );

CREATE TABLE IF NOT EXISTS collections(
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  title VARCHAR(50) NOT NULL,
  description text,
  cover_image_id UUID
  );

CREATE TABLE IF NOT EXISTS image_collections(
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  image_id uuid,
  collection_id uuid,
  CONSTRAINT fk_images FOREIGN KEY(image_id) REFERENCES images(id),
  CONSTRAINT fk_collections FOREIGN KEY(collection_id) REFERENCES collections(id)
  );

INSERT INTO users (username) VALUES
  ('thefirst'),
  ('thesecond')
ON CONFLICT (username) DO NOTHING;
