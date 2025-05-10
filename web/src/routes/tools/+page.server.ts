import type { PageServerLoad } from "./$types";
import type { ProductDB } from "$lib/dbTypes";

export const load: PageServerLoad = async ({ locals }) => {
	const db = locals.db;
	const products = db.prepare("select * from products").all() as ProductDB[];

	return {
		products,
	};
};
