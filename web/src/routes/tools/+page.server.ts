import type { PageServerLoad } from "./$types";
import { getProducts } from "$lib/dbTypes";

export const load: PageServerLoad = async ({ locals }) => {
	const db = locals.db;
	const products = getProducts(db);

	return {
		products,
	};
};
