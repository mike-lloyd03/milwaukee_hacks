import productsRaw from "$lib/products.json";
import { type Product } from "$lib/types";
import type { ProductDB } from "./dbTypes";

// Returns the products available in the specified promo
export function getProducts(promo: string): Product[] {
	const products = productsRaw as Record<string, Product[]>;
	return products[promo].sort((a, b) => a.name.localeCompare(b.name));
}

export function formatCurrency(value: number) {
	const formatter = new Intl.NumberFormat("en-US", {
		style: "currency",
		currency: "USD",
		trailingZeroDisplay: "stripIfInteger",
	});
	return formatter.format(value);
}

export function formatPercent(value: number) {
	const formatter = new Intl.NumberFormat("en-US", {
		style: "percent",
		roundingIncrement: 1,
	});
	return formatter.format(value);
}

export function simplifyName(product: ProductDB): string {
	const removeStr = ["18V", "Lithium-Ion", "Cordless", "12V", "12-Volt"];
	let newName = product.product_label;
	for (const s of removeStr) {
		newName = newName.replaceAll(s, "");
	}

	if (newName.includes("FUEL")) {
		newName = newName.replace("Brushless", "");
	}
	return newName;
}

export function uniqueProducts(l: Product[]): Product[] {
	const r: Product[] = [];
	for (const i of l) {
		if (!r.map((p) => p.name).includes(i.name)) {
			r.push(i);
		}
	}
	return r;
}
