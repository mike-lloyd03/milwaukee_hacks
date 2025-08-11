import type { PageServerLoad } from "./$types";
import { getProducts, type ProductDB, type PromotionDB } from "$lib/dbTypes";

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

	const products = getProducts(db, itemIDs);

	return {
		products,
		promo,
	};
};
