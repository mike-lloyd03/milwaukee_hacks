import type { Handle } from "@sveltejs/kit";
import Database from "better-sqlite3";

export const handle: Handle = async ({ event, resolve }) => {
	if (!event.locals.db) {
		const db = new Database("data/data.db", { verbose: console.log });
		db.pragma("journal_mode = WAL");
		event.locals.db = db;
	}
	const resp = await resolve(event);
	return resp;
};
