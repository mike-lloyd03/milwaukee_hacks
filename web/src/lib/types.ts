import { type Database } from "better-sqlite3";

export interface Product {
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
	original?: number;
	promotion?: {
		promotion_tag?: string;
		type?: string;
		description?: {
			short_desc?: string;
			long_desc?: string;
		};
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

	query += " order by json_extract(pricing, '$.value') desc";

	const result = db.prepare(query).all(ids ?? {}) as unknown[];
	const products = result.map((promo) => objToProduct(promo));

	return products;
}

export function getProduct(db: Database, id: string) {
	const product = db
		.prepare("select * from products where item_id = ?")
		.get(id);

	return objToProduct(product);
}

function objToProduct(obj: unknown): Product {
	if (obj == undefined) {
		throw "product not found";
	}

	const product = obj as Product;

	if (typeof product.pricing == "string") {
		product.pricing = JSON.parse(product.pricing);
	}

	if (typeof product.image_primary_sizes == "string") {
		product.image_primary_sizes = JSON.parse(product.image_primary_sizes);
	}

	if (typeof product.image_secondary_sizes == "string") {
		product.image_secondary_sizes = JSON.parse(product.image_secondary_sizes);
	}

	return product;
}

export interface Promotion {
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
	max_allowed_reward_amount?: number;
	max_purchase_quantity?: number;
	min_purchase_quantity?: number;
	max_purchase_amount?: number;
	min_purchase_amount?: number;
	reward_amount_per_item?: number;
	reward_amount_per_order?: number;
	reward_fixed_price?: number;
	reward_percent?: number;
}

export interface EligibilityCriterion {
	item_group: string;
	categories: string[];
	item_ids: string;
	min_purchase_amount?: number;
	min_purchase_quantity?: number;
}

export function getPromotions(db: Database, ids?: string[]): Promotion[] {
	let query = "select * from promotions";

	if (ids) {
		query += " where promotion_id in (";
		query += ids.map(() => "?").join(", ");
		query += ")";
	}

	const result = db.prepare(query).all(ids ?? {}) as unknown[];
	const promos = result.map((promo) => objToPromo(promo));

	return promos;
}

export function getPromotion(db: Database, id: string): Promotion {
	const promo = db
		.prepare("select * from promotions where promotion_id = ?")
		.get(id);

	return objToPromo(promo);
}

function objToPromo(obj: unknown): Promotion {
	if (obj == undefined) {
		throw "promotion not found";
	}

	const promo = obj as Promotion;

	if (promo.reward_tiers) {
		promo.reward_tiers = JSON.parse(promo.reward_tiers.toString());
	}

	if (promo.eligibility_criteria) {
		promo.eligibility_criteria = JSON.parse(
			promo.eligibility_criteria.toString(),
		);
	}

	if (promo.item_ids) {
		promo.item_ids = JSON.parse(promo.item_ids);
	}

	return promo;
}
