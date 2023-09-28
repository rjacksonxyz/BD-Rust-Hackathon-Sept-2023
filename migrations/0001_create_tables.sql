-- migrations/{timestamp}_create_tables.sql
-- Create Tables
CREATE TABLE users(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    user_id TEXT NOT NULL UNIQUE,
    subscribed_at timestamptz NOT NULL
);

CREATE TABLE transaction_history(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    user_id TEXT NOT NULL,
    asset_ticker TEXT NOT NULL,
    price FLOAT NOT NULL,
    quantity INTEGER NOT NULL,
    total_amount FLOAT NOT NULL,
    purchased_at timestamptz NOT NULL,
    order_type TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);
