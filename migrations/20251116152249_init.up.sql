-- Add up migration script here

CREATE EXTENSION btree_gist;

CREATE TYPE user_role AS ENUM ('player', 'business', 'admin');

CREATE TYPE sport AS ENUM ('football', 'padel', 'basketball');

CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  username VARCHAR(128) UNIQUE NOT NULL,
  password_hash BYTEA NOT NULL,
  password_salt BYTEA NOT NULL,
  role user_role NOT NULL
);

CREATE TABLE player_profiles (
  user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
  first_name VARCHAR(128) NOT NULL,
  last_name VARCHAR(128) NOT NULL,
  preferred_sports sport[] NOT NULL DEFAULT '{}'
);

CREATE TABLE business_profiles (
  user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
  display_name VARCHAR(128) UNIQUE NOT NULL
);

CREATE TABLE pitches (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  owner_id UUID NOT NULL REFERENCES business_profiles(user_id) ON DELETE CASCADE,
  display_name VARCHAR(128) NOT NULL,
  sport sport NOT NULL,
  UNIQUE (owner_id, display_name)
);

CREATE TABLE timeslots (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  pitch_id UUID NOT NULL REFERENCES pitches(id) ON DELETE CASCADE,
  time_range TSTZRANGE NOT NULL,
  is_booked BOOLEAN NOT NULL DEFAULT FALSE,
  EXCLUDE USING gist (
    pitch_id WITH =,
    time_range WITH &&
  )
);

CREATE TABLE bookings (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  timeslot_id UUID UNIQUE NOT NULL REFERENCES timeslots(id),
  booked_by UUID NOT NULL REFERENCES player_profiles(user_id)
);
