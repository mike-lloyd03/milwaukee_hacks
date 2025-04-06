import type { PageServerLoad } from "./$types";
import type { ProductDB, PromotionDB } from "$lib/dbTypes";

export const load: PageServerLoad = async ({ locals }) => {
	const db = locals.db;
	const promo = db
		.prepare("select * from promotions where item_id = ?")
		.get("326279262") as PromotionDB;
	const itemIDs: string[] = JSON.parse(promo.item_ids);

	let query = "select * from products where item_id in (";
	query += itemIDs.map(() => "?").join(", ");
	query += ") order by pricing_value";

	const products: ProductDB[] = db.prepare(query).all(itemIDs) as ProductDB[];

	return {
		products: products,
	};
};
