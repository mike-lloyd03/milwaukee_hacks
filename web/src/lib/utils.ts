import productsRaw from "$lib/products.json";
import { type Product } from "$lib/types";

// Returns the products available in the specified promo
export function getProducts(promo: string): Product[] {
	const products = productsRaw as Record<string, Product[]>;
	return products[promo];
}

export function formatCurrency(value: number) {
	const formatter = new Intl.NumberFormat("en-US", {
		style: "currency",
		currency: "USD",
		trailingZeroDisplay: "stripIfInteger",
	});
	return formatter.format(value);
}
