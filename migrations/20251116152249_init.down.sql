-- Add down migration script here

DROP TABLE bookings, timeslots, pitches, business_profiles, player_profiles, users;

DROP TYPE sport, user_role;

DROP EXTENSION btree_gist;
