CREATE TABLE players(
  player_id UUID PRIMARY KEY,
  version UUID NOT NULL,
  created TIMESTAMP WITH TIME ZONE NOT NULL,
  updated TIMESTAMP WITH TIME ZONE NOT NULL,

  email TEXT NOT NULL,
  display_name TEXT NOT NULL,
  authentication JSONB NOT NULL
);

ALTER TABLE players ADD CONSTRAINT players_email_key UNIQUE (email);
