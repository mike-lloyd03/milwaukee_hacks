import type { PageServerLoad } from "./$types";
import type { PromotionDB } from "$lib/dbTypes";

export const load: PageServerLoad = async ({ locals }) => {
	const db = locals.db;
	const promos = db.prepare("select * from promotions").all() as PromotionDB[];
	promos.map((promo) => {
		if (promo.reward_tiers) {
			promo.reward_tiers = JSON.parse(promo.reward_tiers.toString());
		}
		return promo;
	});

	return {
		promos,
	};
};
