-- companies
DELETE FROM companies;
INSERT INTO companies (name) VALUES ('Al Forsan');
INSERT INTO companies (name) VALUES ('Al Joker');
INSERT INTO companies (name) VALUES ('Al Abtal');

-- users
DELETE FROM users;
INSERT INTO users (username, first_name, last_name, password_hash, password_salt) VALUES (
  'ahmad.alsaleh',
  'Ahmad',
  'Alsaleh',
  '\x30b6d7def1889b5cdd37e3572195b5e33698fdaa62f5a1b0af21022d1f368c3f', -- the password is: passme
  '\x6c73ad746c650ed4c9613a36809725058dfe02069989d66795903e679ca3a104'
);
INSERT INTO users (username, first_name, last_name, password_hash, password_salt) VALUES (
  'mohammed.hassan',
  'Mohammed',
  'Hassan',
  '\x62131cd7df415ac6d4adb877a2965b4a11f6fb53bc9762eee1c3b97cc0118ce0', -- the password is: passme
  '\xa2f4c2fcc2857723cf9f0d7b3c2765ada7bee3378b9fbedbfc3e6aac995faa51'
);

