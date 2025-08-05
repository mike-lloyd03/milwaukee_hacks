import type { PageServerLoad } from "./$types";
import type { ProductDB, PromotionDB } from "$lib/dbTypes";

export const load: PageServerLoad = async ({ locals, params }) => {
	const db = locals.db;
	const promo = db
		.prepare("select * from promotions where promotion_id = ?")
		.get(params.promoId) as PromotionDB;

	if (promo.reward_tiers) {
		promo.reward_tiers = JSON.parse(promo.reward_tiers.toString());
	}

	promo.eligibility_criteria = JSON.parse(
		promo.eligibility_criteria.toString(),
	);

	const itemIDs: string[] = promo.eligibility_criteria
		.map((ec) => ec.itemIds)
		.flat();

	let query = "select * from products where item_id in (";
	query += itemIDs.map(() => "?").join(", ");
	query += ") order by pricing_value";

	const products: ProductDB[] = db.prepare(query).all(itemIDs) as ProductDB[];

	return {
		products,
		promo,
	};
};
