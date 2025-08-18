import { describe, it, expect } from "vitest";
import { indexSetToCharGroup } from "./utils";

describe("indexSetToCharGroup", () => {
	it("should return the string if `indices` is undefined", () => {
		const str = "Test phrase";
		const indices = undefined;
		const expected = [{ chars: "Test phrase", bold: false }];
		expect(indexSetToCharGroup(str, indices)).toEqual(expected);
	});

	it("should group a string into bold and non-bold segments", () => {
		const str = "SvelteKit";
		const indices = new Set([6, 7, 8]); // Highlight "Kit"
		const expected = [
			{ chars: "Svelte", bold: false },
			{ chars: "Kit", bold: true },
		];
		console.log(indexSetToCharGroup(str, indices));
		expect(indexSetToCharGroup(str, indices)).toEqual(expected);
	});

	// Test Case 2: Highlighting starts at the beginning
	it("should handle highlighting that starts at index 0", () => {
		const str = "Hello World";
		const indices = new Set([0, 1, 2, 3, 4]); // Highlight "Hello"
		const expected = [
			{ chars: "Hello", bold: true },
			{ chars: " World", bold: false },
		];
		expect(indexSetToCharGroup(str, indices)).toEqual(expected);
	});

	// Test Case 3: Non-contiguous highlighting
	it("should handle multiple, separate bold groups", () => {
		const str = "Testing 1, 2, 3";
		const indices = new Set([0, 1, 2, 10, 11, 12]); // Highlight "Tes" and "2, "
		const expected = [
			{ chars: "Tes", bold: true },
			{ chars: "ting 1,", bold: false },
			{ chars: " 2,", bold: true },
			{ chars: " 3", bold: false },
		];
		expect(indexSetToCharGroup(str, indices)).toEqual(expected);
	});

	// --- Edge Cases ---

	// Test Case 4: Empty string input
	it("should return an empty array for an empty string", () => {
		const str = "";
		const indices = new Set([0, 1]);
		expect(indexSetToCharGroup(str, indices)).toEqual([]);
	});

	it("should return the whole string as non-bold for an empty indices set", () => {
		const str = "Full String";
		const indices = new Set<number>();
		expect(indexSetToCharGroup(str, indices)).toEqual([
			{ chars: "Full String", bold: false },
		]);
	});

	// Test Case 6: All characters are highlighted
	it("should return the whole string as a single bold segment if all indices are present", () => {
		const str = "BOLD";
		const indices = new Set([0, 1, 2, 3]);
		expect(indexSetToCharGroup(str, indices)).toEqual([
			{ chars: "BOLD", bold: true },
		]);
	});
});
