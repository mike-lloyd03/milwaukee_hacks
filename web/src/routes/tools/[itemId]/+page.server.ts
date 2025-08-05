import type { PageServerLoad } from "./$types";
import type { ProductDB, ProductPromotionDB, PromotionDB } from "$lib/dbTypes";
import { fail } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ locals, params }) => {
	if (!isNaN(Number(params.itemId))) {
		fail(400, { message: "itemId is not numeric" });
	}

	const db = locals.db;
	const product = db
		.prepare("select * from products where item_id = ?")
		.get(params.itemId) as ProductDB;

	const product_promotions = db
		.prepare("select * from product_promotions where product_id = ?")
		.all(params.itemId) as ProductPromotionDB[];

	const product_promotion_ids = product_promotions.map((p) => p.promotion_id);

	let statement = "select * from promotions where promotion_id in (";
	statement += product_promotion_ids.map(() => "?").join(", ");
	statement += ")";

	const promos = db
		.prepare(statement)
		.all(product_promotion_ids) as PromotionDB[];

	return {
		product,
		promos,
	};
};
