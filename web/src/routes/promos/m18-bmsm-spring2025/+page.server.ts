import type { PageServerLoad } from "./$types";
import type { ProductDB } from "$lib/dbTypes";

export const load: PageServerLoad = async ({ locals }) => {
	const db = locals.db;
	const products: ProductDB[] = db.prepare("select * from products").all();

	return {
		products: products,
	};
};
