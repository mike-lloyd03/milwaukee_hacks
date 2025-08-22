CREATE TABLE runs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    total_products INTEGER NOT NULL,
    total_promotions INTEGER NOT NULL,
    start_time INTEGER NOT NULL,
    end_time INTEGER NOT NULL,
    duration INTEGER NOT NULL
)
