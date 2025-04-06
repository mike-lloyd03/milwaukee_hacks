CREATE TABLE temp_promotions (
    promotion_id TEXT PRIMARY KEY,
    name TEXT NULL,
    item_id TEXT NULL,
    experience_tag TEXT,
    sub_experience_tag TEXT,
    long_description TEXT,
    short_description TEXT,
    start_date TEXT,
    end_date TEXT,
    item_group TEXT,
    categories JSON,
    item_ids JSON ,
    eligibility_min_purchase_amount REAL,
    eligibility_min_purchase_quantity REAL,
    max_allowed_reward_amount REAL,
    max_purchase_quantity REAL,
    min_purchase_amount REAL,
    min_purchase_quantity REAL,
    reward_amount_per_item REAL,
    reward_amount_per_order REAL,
    reward_fixed_price REAL,
    reward_percent REAL
);

INSERT INTO temp_promotions (promotion_id, experience_tag, sub_experience_tag, long_description, short_description, start_date, end_date, item_group, categories, item_ids, eligibility_min_purchase_amount, eligibility_min_purchase_quantity, max_allowed_reward_amount, max_purchase_quantity, min_purchase_amount, min_purchase_quantity, reward_amount_per_item, reward_amount_per_order, reward_fixed_price, reward_percent, name, item_id)
SELECT promotion_id, experience_tag, sub_experience_tag, long_description, short_description, start_date, end_data, item_group, categories, item_ids, eligibility_min_purchase_amount, eligibility_min_purchase_quantity, max_allowed_reward_amount, max_purchase_quantity, min_purchase_amount, min_purchase_quantity, reward_amount_per_item, reward_amount_per_order, reward_fixed_price, reward_percent, name, item_id
FROM promotions;

DROP TABLE promotions;

ALTER TABLE temp_promotions RENAME TO promotions;
