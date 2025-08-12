import type { PageServerLoad } from "./$types";
import { getPromotions, type PromotionDB } from "$lib/dbTypes";

export const load: PageServerLoad = async ({ locals }) => {
	const db = locals.db;
	const promos = getPromotions(db);

	return {
		promos,
	};
};
