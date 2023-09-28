-- migrations/{timestamp}_populate_users.sql
-- Populating users
INSERT INTO users (
    id,
    email,
    name,
    user_id,
    subscribed_at
)
VALUES
    ('783aaaba-9392-412d-99db-7fc9e10debf8', 'law@email.com', 'Lawrence','law','2016-01-25T10:10:10.555555'),
    ('a2e06db1-7267-4d42-8989-f1aeb1ae4a25', 'rob@email.com', 'Robert','rob','2016-01-25T10:10:10.555555');
