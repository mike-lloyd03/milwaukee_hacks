import type { PageServerLoad } from "./$types";
import type { ProductDB } from "$lib/dbTypes";
import { fail } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ locals, params }) => {
	if (!isNaN(Number(params.itemId))) {
		fail(400, { message: "itemId is not numeric" });
	}

	const db = locals.db;
	const product = db
		.prepare("select * from products where item_id = ?")
		.get(params.itemId) as ProductDB;

	return {
		product,
	};
};
