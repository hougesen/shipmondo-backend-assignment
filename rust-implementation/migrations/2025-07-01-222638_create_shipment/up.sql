CREATE TABLE shipments (
    id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    price TEXT NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users (id)
);
