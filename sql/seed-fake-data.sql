-- this file is used for testing purposes only

DELETE FROM bookings;
DELETE FROM timeslots;
DELETE FROM pitches;
DELETE FROM business_profiles;
DELETE FROM player_profiles;
DELETE FROM users;

INSERT INTO users (username, password_hash, password_salt, role)
VALUES
  (
    'player_1',
    '\x30c790d3462f3dfbd5e32d7c308c5d354c9d0e65bd31250208cc6fe517966910', -- the password is: user_1_password
    '\x6032a5a9886c4ef8ef9dcc7c8eb66eb5c4adce2996403bb2cb70fca3b8153a27',
    'player'
  ),
  (
    'player_2',
    '\xdf8c35e3dd4a813c235ac06ac4141ba454873d0621eaaf408db042cd85b32d22', -- the password is: user_2_password
    '\x009f7ee8b92448cb26db9e3a08765cf456aa030d9a71aaed55a4598ae432389f',
    'player'
  ),
  (
    'business_1',
    '\xe5492aa81806796b4f2127a702860f6bc9074b49c06983151f7f0cb26f3f535a', -- the password is: business_1_password
    '\x6cb77ff14717f76ec2dc6a431f2e7483de8c6a411043c4357065ae3fa0738f4a',
    'business'
  ),
  (
    'business_2',
    '\x3e25a17318adce535c262e24895c98b6725ca123bd968e50480a275a59e671bf', -- the password is: business_2_password
    '\xb24f91a3914b017bdc9e7ba00bc5c0ae160d03e87bc627511d828de58c6c65e9',
    'business'
  ),
  (
    'admin',
    '\xe82b0a620aaf49ecd6ab946273546b41c96f401b4729bb917addc9ae0a640c7e', -- the password is: admin
    '\x7d2bd06160c0c3a4d55209a2737ef3bd317c2d8e00d5a881b625fe967dc3c220',
    'admin'
  );

INSERT INTO player_profiles (user_id, first_name, last_name, preferred_sports)
VALUES
  ((SELECT id FROM users WHERE username = 'player_1'), 'player_1_first', 'player_1_last', '{football}'),
  ((SELECT id FROM users WHERE username = 'player_2'), 'player_2_first', 'player_2_last', '{basketball, padel}');

INSERT INTO business_profiles (user_id, display_name)
VALUES
  ((SELECT id FROM users WHERE username = 'business_1'), 'business_1_display'),
  ((SELECT id FROM users WHERE username = 'business_2'), 'business_2_display');

INSERT INTO pitches (owner_id, display_name, sport)
VALUES
  ((SELECT id FROM users WHERE username = 'business_1'), 'football_pitch_1', 'football'),
  ((SELECT id FROM users WHERE username = 'business_1'), 'football_pitch_2', 'football'),
  ((SELECT id FROM users WHERE username = 'business_2'), 'basketball_pitch_1', 'basketball'),
  ((SELECT id FROM users WHERE username = 'business_2'), 'padel_pitch_1', 'padel');

INSERT INTO timeslots (pitch_id, time_range, is_booked)
VALUES
  ((SELECT id FROM pitches WHERE display_name = 'football_pitch_1'), tstzrange('2025-03-30 17:00+04', '2025-03-30 18:00+04'), FALSE),
  ((SELECT id FROM pitches WHERE display_name = 'football_pitch_1'), tstzrange('2025-03-30 18:00+04', '2025-03-30 19:00+04'), FALSE),
  ((SELECT id FROM pitches WHERE display_name = 'football_pitch_1'), tstzrange('2025-03-20 17:00+04', '2025-03-20 18:00+04'), TRUE),
  ((SELECT id FROM pitches WHERE display_name = 'basketball_pitch_1'), tstzrange('2025-03-20 17:00+04', '2025-03-20 18:00+04'), TRUE);

INSERT INTO bookings (timeslot_id, booked_by)
VALUES
  ((SELECT id from timeslots WHERE is_booked ORDER BY id OFFSET 0 LIMIT 1), (SELECT id FROM users WHERE username = 'player_1')),
  ((SELECT id from timeslots WHERE is_booked ORDER BY id OFFSET 1 LIMIT 1), (SELECT id FROM users WHERE username = 'player_2'));
