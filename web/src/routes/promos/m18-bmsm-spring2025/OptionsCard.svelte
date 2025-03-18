<script lang="ts">
	import { bmsm, Cart } from '$lib/pkg/algorithm';
	import type { ProductDB } from '$lib/dbTypes';
	import { Card, NumberInput, Label, Heading, Button, Tooltip, Input } from 'flowbite-svelte';

	interface Props {
		products: ProductDB[];
		requiredProducts: string[];
		carts: Cart[];
	}

	let { products, requiredProducts = $bindable(), carts = $bindable() }: Props = $props();

	let productsFilter = $state('');
	let cartSize = $state(4);
	let minTotal = $state(1000);
	let selectedProducts: string[] = $state([]);

	function rowDisabled(product_label: string): boolean {
		return !selectedProducts.includes(product_label) && selectedProducts.length >= cartSize;
	}

	function cartSizeRange() {
		if (cartSize < 2) {
			cartSize = 2;
		} else if (cartSize > 6) {
			cartSize = 6;
		}
	}
	function minTotalRange() {
		if (minTotal < 100) {
			minTotal = 100;
		} else if (minTotal > 10000) {
			minTotal = 10000;
		}
	}

	function getCarts(event: MouseEvent) {
		event.preventDefault();
		requiredProducts = selectedProducts;
		const productData = products.map((p: ProductDB) => {
			return { name: p.product_label, price: p.pricing_value };
		});
		const allCarts = bmsm(productData, cartSize, minTotal, requiredProducts);
		carts = allCarts.slice(0, 10);
	}
</script>

<Card size="md">
	<Heading tag="h5">Options</Heading>
	<div class="my-4 flex flex-col gap-4">
		<Label>
			Products
			<Input class="mb-1" bind:value={productsFilter} placeholder="Search" />
			<div
				class="flex h-64 flex-col overflow-y-auto rounded-md border border-gray-300 p-2 dark:border-gray-500"
			>
				{#each products as product (product.product_label)}
					{#if product.product_label.toLowerCase().includes(productsFilter.toLowerCase())}
						<label
							class="flex items-center py-2 text-sm font-medium {rowDisabled(product.product_label)
								? 'text-gray-500 dark:text-gray-600'
								: 'text-gray-900 dark:text-gray-300'} "
						>
							<input
								class="text-primary-600 focus:ring-primary-500 dark:focus:ring-primary-600 me-2 h-4 w-4 rounded border-gray-300 bg-gray-100 focus:ring-2 dark:border-gray-500 dark:bg-gray-600 dark:ring-offset-gray-800"
								type="checkbox"
								name="requiredProduct"
								value={product.product_label}
								bind:group={selectedProducts}
								disabled={rowDisabled(product.product_label)}
							/>
							{product.product_label}
						</label>
					{/if}
				{/each}
			</div>
			<Tooltip>Items which must appear in the cart (the items you want to hack)</Tooltip>
		</Label>

		<Label>
			Max Cart Size
			<NumberInput onchange={cartSizeRange} bind:value={cartSize} />
			<Tooltip>The maximum number of items in the cart. 4-5 is usually a good value.</Tooltip>
		</Label>

		<Label>
			Minimum Cart Total
			<NumberInput prefix="$" onchange={minTotalRange} bind:value={minTotal} />
			<Tooltip>The minimum total cost of the items in the cart</Tooltip>
		</Label>

		<Button color="red" onclick={getCarts}>Calculate</Button>
	</div>
</Card>
