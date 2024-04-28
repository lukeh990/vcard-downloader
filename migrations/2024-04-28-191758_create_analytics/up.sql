-- Your SQL goes here
CREATE TABLE IF NOT EXISTS analytics (
    uuid TEXT NOT NULL UNIQUE,
    timestamp INTEGER NOT NULL,
    card_uuid TEXT NOT NULL,
    PRIMARY KEY (uuid),
    FOREIGN KEY(card_uuid) REFERENCES vcards(uuid)
) WITHOUT ROWID