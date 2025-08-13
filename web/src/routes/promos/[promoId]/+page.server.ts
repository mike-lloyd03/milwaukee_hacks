import type { PageServerLoad } from "./$types";
import { getProducts, getPromotion, type Promotion } from "$lib/types";
import { error } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ locals, params }) => {
	if (isNaN(Number(params.promoId))) {
		error(400, { message: "promoId is not numeric" });
	}

	const db = locals.db;
	let promo: Promotion;

	try {
		promo = getPromotion(db, params.promoId);
	} catch (e) {
		if (typeof e == "string") {
			if (e.includes("not found")) {
				error(404, "Promotion not found");
			}
		} else {
			throw e;
		}
	}

	const itemIDs: string[] = promo!.eligibility_criteria
		.map((ec) => ec.item_ids)
		.flat();

	const products = getProducts(db, itemIDs);

	return {
		products,
		promo: promo!,
	};
};
