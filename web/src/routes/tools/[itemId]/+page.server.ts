import type { PageServerLoad } from "./$types";
import { getProduct, getPromotions, type ProductDB } from "$lib/dbTypes";
import { error } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ locals, params }) => {
	if (isNaN(Number(params.itemId))) {
		return error(400, { message: "itemId is not numeric" });
	}

	const db = locals.db;
	let product: ProductDB;

	try {
		product = getProduct(db, params.itemId);
	} catch (e) {
		if (typeof e == "string") {
			if (e.includes("not found")) {
				error(404, "Product not found");
			}
		} else {
			throw e;
		}
	}

	const product_promotion_ids = product!.pricing.conditional_promotions
		.filter((p) => p.promotion_id != undefined)
		.map((p) => p.promotion_id!.toString());

	const promos = getPromotions(db, product_promotion_ids);

	return {
		product: product!,
		promos,
	};
};
