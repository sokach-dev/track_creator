-- Add up migration script here

-- create creator table in sqliite
-- the symbol maybe have some specific character, so we use TEXT type
CREATE TABLE creator (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mint TEXT NOT NULL UNIQUE, -- mint address
    creator TEXT NOT NULL, -- creator address
    symbol TEXT NOT NULL, -- symbol
    created_at INTEGER NOT NULL -- created time
);
