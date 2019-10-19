-- Your SQL goes here
CREATE TABLE sessions (
  Id VARCHAR PRIMARY KEY,
  playerNumber VARCHAR NOT NULL,
  total_ticks INTEGER NOT NULL,
  game_name VARCHAR NOT NULL,
  table_name VARCHAR NOT NULL,
  player_points INTEGER,
  begin_at TIMESTAMP,
  end_at TIMESTAMP
)