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

export interface PromotionDB {
	promotion_id: string;
	name: string;
	item_id: string;
	experience_tag: string;
	sub_experience_tag: string;
	long_description?: string;
	short_description: string;
	start_date: string;
	end_date: string;
	item_group: string;
	categories: string; // JSON string of string[]
	item_ids: string; // JSON string of string[]
	eligibility_min_purchase_amount: number;
	eligibility_min_purchase_quantity?: number;
	max_allowed_reward_amount?: number;
	max_purchase_quantity?: number;
	min_purchase_amount?: number;
	min_purchase_quantity?: number;
	reward_amount_per_item?: number;
	reward_amount_per_order?: number;
	reward_fixed_price?: number;
	reward_percent?: number;
	reward_tiers?: RewardTier[];
}

export interface RewardTier {
	maxAllowedRewardAmount?: number;
	maxPurchaseQuantity?: number;
	minPurchaseQuantity?: number;
	maxPurchaseAmount?: number;
	minPurchaseAmount?: number;
	rewardAmountPerItem?: number;
	rewardAmountPerOrder?: number;
	rewardFixedPrice?: number;
	rewardPercent?: number;
}
