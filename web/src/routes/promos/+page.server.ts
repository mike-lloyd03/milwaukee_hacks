import type { PageServerLoad } from "./$types";
import { getPromotions, type Promotion } from "$lib/types";

export const load: PageServerLoad = async ({ locals }) => {
	const db = locals.db;
	const promos = getPromotions(db);

	return {
		promos,
	};
};
