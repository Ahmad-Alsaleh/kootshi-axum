-- Add down migration script here

ALTER TABLE users DROP COLUMN role;

DROP TYPE USER_ROLE;
