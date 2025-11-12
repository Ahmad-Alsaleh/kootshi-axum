-- companies
INSERT INTO companies (name) VALUES ('Al Forsan');
INSERT INTO companies (name) VALUES ('Al Joker');
INSERT INTO companies (name) VALUES ('Al Abtal');

-- users
INSERT INTO users (username, first_name, last_name, password_hash, password_salt) VALUES (
  'ahmad.alsaleh',
  'Ahmad',
  'Alsaleh',
  '\x055d9a5d9d14c67281cc23a76752bf116980e111c81e140142a8a8d24e4d4592',
  '\xfd3feba17baebb786e3c1838261aec99160413b30425119dd81484e135b4d5e2'
);
INSERT INTO users (username, first_name, last_name, password_hash, password_salt) VALUES (
  'mohammed.hassan',
  'Mohammed',
  'Hassan',
  '\xb7e9a65e6b64596dfc5a74e9b443e15311fedd2c2f7d8217b6a6de0bc4e6105c',
  '\x733b950cc47f5123e80fac981e79d7ef55bec2cb24d643cdd84260d68d17bc95'
);

