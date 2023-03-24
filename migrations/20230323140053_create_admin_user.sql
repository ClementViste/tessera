-- Create admin user.
INSERT INTO users (user_id, username, password_hash)
VALUES (
        '2061de8c-e783-4ce9-8451-539989a5e7e1',
        'admin',
        '$argon2id$v=19$m=15000,t=2,p=1$c2XBZ2Hrm0G04m+67AoJwg$sF8yP0YrbY9P/rOd7dFz2oArRgkphfEr4y05IdgOsFA'
    );