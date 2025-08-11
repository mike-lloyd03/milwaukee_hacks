import { type Database } from "better-sqlite3";

export interface ProductDB {
	item_id: string;
	brand_name: string;
	product_type: string;
	product_label: string;
	canonical_url: string;
	pricing: Pricing;
	image_primary_url: string;
	image_primary_sizes: string[];
	image_secondary_url: string;
	image_secondary_sizes: string[];
	updated_at: number;
}

export interface Pricing {
	value: number;
	original: number;
	promotion: {
		promotion_tag?: string;
		type?: string;
		description?: string;
		dollar_off: number;
		percentage_off: number;
		savings_center?: string;
		savings_center_promos?: string;
		special_buy_savings?: string;
		special_buy_dollar_off?: number;
		special_buy_percentage_off?: number;
	};
	conditional_promotions: {
		promotion_id?: number;
	}[];
	message?: string;
	special_buy?: string;
}

export function getProducts(db: Database, ids?: string[]) {
	let query = "select * from products";

	if (ids) {
		query += " where item_id in (";
		query += ids.map(() => "?").join(", ");
		query += ")";
	}

	query += " order by json_extract(pricing, '$.pricing.value')";

	const promos = db.prepare(query).all(ids ?? {}) as ProductDB[];

	return promos.map((p) => {
		if (typeof p.pricing == "string") {
			p.pricing = JSON.parse(p.pricing);
		}
		return p;
	});
}

export function getProduct(db: Database, id: string) {
	const promo = db
		.prepare("select * from products where item_id = ?")
		.get(id) as ProductDB;
	if (typeof promo.pricing == "string") {
		promo.pricing = JSON.parse(promo.pricing);
	}
	return promo;
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
	eligibility_criteria: EligibilityCriterion[];
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

export interface EligibilityCriterion {
	itemGroup: string;
	categories: string[];
	itemIds: string;
	minPurchaseAmount?: number;
	minPurchaseQuantity?: number;
}

export interface ProductPromotionDB {
	id: number;
	product_id: string;
	promotion_id: string;
}
