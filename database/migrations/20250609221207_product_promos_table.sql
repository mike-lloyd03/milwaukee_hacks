PRAGMA foreign_keys = ON;

CREATE TABLE product_promotions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    product_id TEXT,
    promotion_id TEXT,
    FOREIGN KEY (product_id) REFERENCES products(item_id) ON DELETE CASCADE,
    FOREIGN KEY (promotion_id) REFERENCES promotions(promotion_id) ON DELETE CASCADE
);
