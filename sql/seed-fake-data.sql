-- this file is used for testing purposes only

INSERT INTO users (id, username, password_hash, password_salt, role)
VALUES
  (
    '00000000-0000-0000-0000-000000000001',
    'player_1',
    '\xb8b1839b5f65c5dd0214aaae35f3dc5fc21ed9e15719450aec736b58fa91a7c8', -- the password is: player_1_password
    '\xb4f340d960f6d1b450794fb1ed8154919dc37784a874ebc4e51cea7148df1398',
    'player'
  ),
  (
    '00000000-0000-0000-0000-000000000002',
    'player_2',
    '\x50669cfbf4e50b0477d727c92b5beba1c48fa5a0ab6c30fde7a5c3b1160c995a', -- the password is: player_2_password
    '\xa6f4fdee669cd20cb338fb29179f776e6bac19bee82bb787ece3a2cbe969c623',
    'player'
  ),
  (
    '00000000-0000-0000-0000-000000000003',
    'business_1',
    '\xe5492aa81806796b4f2127a702860f6bc9074b49c06983151f7f0cb26f3f535a', -- the password is: business_1_password
    '\x6cb77ff14717f76ec2dc6a431f2e7483de8c6a411043c4357065ae3fa0738f4a',
    'business'
  ),
  (
    '00000000-0000-0000-0000-000000000004',
    'business_2',
    '\x3e25a17318adce535c262e24895c98b6725ca123bd968e50480a275a59e671bf', -- the password is: business_2_password
    '\xb24f91a3914b017bdc9e7ba00bc5c0ae160d03e87bc627511d828de58c6c65e9',
    'business'
  ),
  (
    '00000000-0000-0000-0000-000000000005',
    'admin',
    '\xfbd7ffeceb2a47740f805a3a8828f702303a0579160579451e613e8ad350312f', -- the password is: admin_password
    '\x80bb288fefccd696c713d795126d3ec585dae2ddb00648ae57df9d65737509b2',
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

INSERT INTO pitches (id, owner_id, display_name, sport)
VALUES
  ('00000000-0000-0000-0000-000000000006', (SELECT id FROM users WHERE username = 'business_1'), 'football_pitch_1', 'football'),
  ('00000000-0000-0000-0000-000000000007', (SELECT id FROM users WHERE username = 'business_1'), 'football_pitch_2', 'football'),
  ('00000000-0000-0000-0000-000000000008', (SELECT id FROM users WHERE username = 'business_2'), 'basketball_pitch_1', 'basketball'),
  ('00000000-0000-0000-0000-000000000009', (SELECT id FROM users WHERE username = 'business_2'), 'padel_pitch_1', 'padel');

INSERT INTO timeslots (id, pitch_id, time_range, is_booked)
VALUES
  ('00000000-0000-0000-0000-000000000010', (SELECT id FROM pitches WHERE display_name = 'football_pitch_1'), tstzrange('2025-03-30 17:00+04', '2025-03-30 18:00+04'), FALSE),
  ('00000000-0000-0000-0000-000000000011', (SELECT id FROM pitches WHERE display_name = 'football_pitch_1'), tstzrange('2025-03-30 18:00+04', '2025-03-30 19:00+04'), FALSE),
  ('00000000-0000-0000-0000-000000000012', (SELECT id FROM pitches WHERE display_name = 'football_pitch_1'), tstzrange('2025-03-20 17:00+04', '2025-03-20 18:00+04'), TRUE),
  ('00000000-0000-0000-0000-000000000013', (SELECT id FROM pitches WHERE display_name = 'basketball_pitch_1'), tstzrange('2025-03-20 17:00+04', '2025-03-20 18:00+04'), TRUE);

INSERT INTO bookings (id, timeslot_id, booked_by)
VALUES
  ('00000000-0000-0000-0000-000000000014', (SELECT id from timeslots WHERE is_booked ORDER BY id OFFSET 0 LIMIT 1), (SELECT id FROM users WHERE username = 'player_1')),
  ('00000000-0000-0000-0000-000000000015', (SELECT id from timeslots WHERE is_booked ORDER BY id OFFSET 1 LIMIT 1), (SELECT id FROM users WHERE username = 'player_2'));
