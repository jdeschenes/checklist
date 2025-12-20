INSERT INTO users (email)
VALUES ('deschenes.j.m@gmail.com')
ON CONFLICT (email) DO NOTHING;
