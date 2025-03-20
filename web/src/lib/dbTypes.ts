export interface ProductDB {
	item_id: string;
	brand_name: string;
	product_type: string;
	product_label: string;
	canonical_url: string;
	pricing_value: number;
	pricing_original: number;
	promotion_tag?: string;
	promotion_type?: string;
	promotion_description?: string;
	promotion_dollar_off: number;
	promotion_percentage_off: number;
	promotion_savings_center?: string;
	promotion_savings_center_promos?: string;
	promotion_special_buy_savings?: string;
	promotion_special_buy_dollar_off?: number;
	promotion_special_buy_percentage_off?: number;
	pricing_message?: string;
	special_buy?: string;
	image_primary_url: string;
	image_primary_sizes: string[];
	image_secondary_url: string;
	image_secondary_sizes: string[];
}
