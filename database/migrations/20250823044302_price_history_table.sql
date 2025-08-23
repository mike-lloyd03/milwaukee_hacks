CREATE TABLE price_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id TEXT NOT NULL,
    product_label TEXT,
    time INTEGER NOT NULL,
    price REAL NOT NULL
);

CREATE TRIGGER price_history_after_insert
AFTER INSERT ON products
FOR EACH ROW
BEGIN
    INSERT INTO price_history (item_id, product_label, time, price)
    VALUES (
        NEW.item_id,
        NEW.product_label,
        strftime('%s', 'now'),
        json_extract(NEW.pricing, '$.value')
    );
END;

CREATE TRIGGER price_history_after_update
AFTER UPDATE ON products
FOR EACH ROW
WHEN
    json_extract(OLD.pricing, '$.value') <> json_extract(NEW.pricing, '$.value')
    OR OLD.product_label <> NEW.product_label
BEGIN
    INSERT INTO price_history (item_id, product_label, time, price)
    VALUES (
        NEW.item_id,
        NEW.product_label,
        strftime('%s', 'now'),
        json_extract(NEW.pricing, '$.value')
    );
END;
