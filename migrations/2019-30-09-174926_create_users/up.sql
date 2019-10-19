-- Your SQL goes here
CREATE TABLE users (
  Id SERIAL PRIMARY KEY,
  playerNumber VARCHAR NOT NULL,
  firstName VARCHAR NOT NULL,
  lastName VARCHAR NOT NULL,
  tier INTEGER,
  address1 VARCHAR NOT NULL,
  city VARCHAR NOT NULL,
  zip VARCHAR,
  country VARCHAR,
  email VARCHAR,
  id3 VARCHAR,
  isBanned INTEGER,
  latitude FLOAT,
  longitude FLOAT,
  gender INTEGER,
  created TIMESTAMP
)
