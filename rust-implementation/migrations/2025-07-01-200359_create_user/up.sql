-- TODO: add a 3 way unique index (username, password, production)
CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    production BOOLEAN NOT NULL,
    -- NOTE: we could use a timestamp which would provide more information
    is_deleted BOOLEAN NOT NULL DEFAULT 0
);
