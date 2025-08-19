import type { Database } from "better-sqlite3";
import type { Product } from "$lib/types";
// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			db: Database;
		}
		interface PageData {
			product?: Product;
			products?: Product[];
			promo?: PromotionDB;
			promos?: PromotionDB[];
		}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
