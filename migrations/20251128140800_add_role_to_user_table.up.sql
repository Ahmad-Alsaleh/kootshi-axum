-- Add up migration script here

CREATE TYPE USER_ROLE AS ENUM ('player', 'business', 'admin');

ALTER TABLE users ADD role USER_ROLE;
UPDATE users SET role = 'player';
ALTER TABLE users ALTER COLUMN role SET NOT NULL;
