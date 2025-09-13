-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    username TEXT
);

CREATE TABLE IF NOT EXISTS accounts (
    id INTEGER PRIMARY KEY,
    name TEXT,
    initial_balance INTEGER,

    user_id INTEGER,

    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS sellers (
    id INTEGER PRIMARY KEY,
    name TEXT
);

CREATE TABLE IF NOT EXISTS labels (
    id INTEGER PRIMARY KEY,
    name TEXT
);

CREATE TABLE IF NOT EXISTS movements (
    id INTEGER PRIMARY KEY,
    value INTEGER,
    detail TEXT,

    account_id INTEGER,
    seller_id INTEGER,

    FOREIGN KEY(account_id) REFERENCES accounts(id),
    FOREIGN KEY(seller_id) REFERENCES sellers(id)
);

CREATE TABLE IF NOT EXISTS details (
    id INTEGER PRIMARY KEY,
    value INTEGER,
    detail INTEGER,

    movement_id INTEGER,

    FOREIGN KEY(movement_id) REFERENCES movements(id)
);

CREATE TABLE IF NOT EXISTS movements_labels (
    label_id INTEGER,
    movement_id INTEGER,

    FOREIGN KEY(label_id) REFERENCES labels(id),
    FOREIGN KEY(movement_id) REFERENCES movements(id)
);
