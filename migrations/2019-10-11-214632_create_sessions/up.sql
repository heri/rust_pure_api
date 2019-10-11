-- Your SQL goes here
CREATE TABLE sessions (
  id VARCHAR PRIMARY KEY,
  player_number VARCHAR NOT NULL,
  total_ticks INTEGER NOT NULL,
  game_name VARCHAR NOT NULL,
  table_name VARCHAR NOT NULL,
  player_points INTEGER,
  begin_at TIMESTAMP,
  end_at TIMESTAMP
)