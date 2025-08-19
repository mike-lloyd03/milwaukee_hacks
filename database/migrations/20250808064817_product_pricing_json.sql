CREATE TABLE products_new (
    item_id TEXT PRIMARY KEY,
    brand_name TEXT,
    product_type TEXT,
    product_label TEXT,
    canonical_url TEXT,
    pricing JSON,
    image_primary_url TEXT,
    image_primary_sizes JSON,
    image_secondary_url TEXT,
    image_secondary_sizes JSON,
    updated_at INTEGER
);

INSERT INTO products_new (
    item_id,
    brand_name,
    product_type,
    product_label,
    canonical_url,
    image_primary_url,
    image_primary_sizes,
    image_secondary_url,
    image_secondary_sizes,
    updated_at
) SELECT
    item_id,
    brand_name,
    product_type,
    product_label,
    canonical_url,
    image_primary_url,
    image_primary_sizes,
    image_secondary_url,
    image_secondary_sizes,
    updated_at
FROM products;

DROP TABLE products;
ALTER TABLE products_new RENAME TO products;
