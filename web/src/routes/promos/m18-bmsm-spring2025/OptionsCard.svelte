<script lang="ts">
	import { bmsm, Cart } from '$lib/pkg/algorithm';
	import type { ProductDB } from '$lib/dbTypes';
	import {
		Card,
		NumberInput,
		Label,
		Heading,
		Button,
		Tooltip,
		Input,
		Modal
	} from 'flowbite-svelte';
	import { formatCurrency } from '$lib/utils';

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
	let optionsOpen = $state(false);

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

<Card size="xl" class="mx-auto">
	<Heading tag="h5">Select Products</Heading>
	<div class="my-4 flex flex-col gap-4">
		<div>
			<Input class="mx-auto mb-4 max-w-96" bind:value={productsFilter} placeholder="Search" />
			<div
				class="flex h-96 flex-col overflow-y-auto rounded-md border border-gray-300 p-2 dark:border-gray-500"
			>
				{#each products as product (product.product_label)}
					{#if product.product_label.toLowerCase().includes(productsFilter.toLowerCase())}
						<div class="my-2 rounded-md bg-gray-200 p-1 dark:bg-gray-700">
							<label
								class="flex items-center py-2 text-sm font-medium {rowDisabled(
									product.product_label
								)
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
								<div class="flex w-full items-center justify-between">
									<div class="flex items-center">
										<img
											src={product.image_primary_url.replace('<SIZE>', '65')}
											alt="tool"
											class="rounded-md"
										/>
										<a
											href={`https://www.homedepot.com${product.canonical_url}`}
											class="mx-2 hover:underline"
											target="_blank">{product.product_label}</a
										>
									</div>
									<div class="m-0 md:mr-4">
										{formatCurrency(product.pricing_value)}
									</div>
								</div>
							</label>
						</div>
					{/if}
				{/each}
			</div>
		</div>

		<div class="mx-auto flex max-w-md gap-2">
			<Button color="alternative" onclick={() => (optionsOpen = true)}>Options</Button>
			<Button color="red" onclick={getCarts}>Calculate</Button>
		</div>
	</div>
</Card>

<Modal title="Options" bind:open={optionsOpen} autoclose>
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
</Modal>
