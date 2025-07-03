CREATE TABLE user_balances (
    user_id INTEGER NOT NULL,
    amount REAL NOT NULL DEFAULT 0,
    currency_code TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id) REFERENCES users (id)
);
