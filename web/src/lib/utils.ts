import type { ProductDB } from "./dbTypes";
import type { Product } from "./pkg/algorithm";

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

export function simplifyName(name: string): string {
	const removeStr = [
		"18V",
		"18-Volt",
		"18-V",
		"Lithium-Ion",
		"Cordless",
		"12V",
		"12-Volt",
		"12 Volt",
	];
	let newName = name;
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

export function isFuture(dateString: string): boolean {
	const [month, day, year] = dateString.split("/").map(Number);
	const inputDate = new Date(year, month - 1, day);
	inputDate.setHours(23, 59, 59);
	const today = new Date();

	return inputDate > today;
}
