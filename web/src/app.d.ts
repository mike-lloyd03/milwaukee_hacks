import type { Database } from "better-sqlite3";
import type { ProductDB } from "$lib/dbTypes";
// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			db: Database;
		}
		interface PageData {
			product?: ProductDB;
			products?: ProductDB[];
			promo?: PromotionDB;
			promos?: PromotionDB[];
		}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
