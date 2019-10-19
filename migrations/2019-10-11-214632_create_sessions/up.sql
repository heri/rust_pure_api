-- Your SQL goes here
CREATE TABLE sessions (
  Id VARCHAR PRIMARY KEY,
  playerNumber VARCHAR NOT NULL,
  totalTicks INTEGER NOT NULL,
  gameName VARCHAR NOT NULL,
  tableName VARCHAR NOT NULL,
  playerPoints INTEGER,
  begin_at TIMESTAMP,
  end_at TIMESTAMP
)