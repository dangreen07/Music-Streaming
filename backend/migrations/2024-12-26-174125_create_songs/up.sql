-- Your SQL goes here
CREATE TABLE IF NOT EXISTS songs (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    title VARCHAR NOT NULL,
    artist VARCHAR NOT NULL,
    album VARCHAR NOT NULL,
    duration INT NOT NULL,
    num_samples INT NOT NULL
)