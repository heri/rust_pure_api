-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  player_number VARCHAR NOT NULL,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  tier INTEGER,
  address1 VARCHAR NOT NULL,
  city VARCHAR NOT NULL,
  zip VARCHAR,
  country VARCHAR,
  email VARCHAR,
  id3 VARCHAR,
  is_banned INTEGER,
  latitude FLOAT,
  longitude FLOAT,
  gender INTEGER,
  created TIMESTAMP
)
