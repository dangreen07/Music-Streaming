-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  username VARCHAR NOT NULL,
  permissions VARCHAR NOT NULL,
  password_hash VARCHAR NOT NULL,
  song_id uuid,
  CONSTRAINT fk_song_id
    FOREIGN KEY (song_id)
    REFERENCES songs(id)
    ON DELETE SET NULL
)